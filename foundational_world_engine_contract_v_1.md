# Foundational World Engine Contract v1

This document consolidates and formalizes the foundational design decisions for the **world-state machine ↔ rules engine contract**, with a focus on **speed, determinism, extensibility, and teachability**.

This is the *authoritative reference* for v1 of the system. Everything else (VTT, 3D sync, narrative, character gen, tooling) builds **on top of** this foundation, not *inside* it.

---

## 1. Design Goals (Non-Negotiable)

- **Fast hot loop**: predictable performance per tick
- **Deterministic**: identical input bytes produce identical output
- **Pure rules**: rules engine is a pure function
- **Minimal authority**: store only facts, never views
- **Extensible**: new features arrive as components or projections
- **Debuggable**: replay, audit, divergence analysis are possible

Anti-goals:
- No god objects
- No universal "everything container"
- No queries or IO inside rule evaluation

---

## 2. Core Architecture Overview

### Authoritative Layer
- **World Store**: entity + component data
- **Timeline**: ordered ticks of patches (ops)
- **Rules Engine**: evaluates one tick at a time

### Derived Layer (Cornucopia)
These are **projections**, not authoritative state:

- CharacterGenView
- VTTSceneView
- NarrativeSummaryView
- 3DEngineSyncView
- AuditLogView

Projections:
- are read-only
- can be cached or recomputed
- may be rich and domain-specific
- do *not* feed back into the hot loop directly

---

## 3. Vocabulary (Precise Meanings)

### Entity
An opaque identifier:
```text
EntityId = u64
```

### Component
A **typed bundle of facts** about an entity.

Examples:
- Transform2D
- Stats
- Inventory

Components store *facts*, never UI, narrative, or assets.

### Projection
A **derived read model** built from authoritative state + timeline.

Examples:
- A VTT scene graph
- A 3D engine sync payload
- A narrative recap paragraph

If a projection is wrong, it is deleted and rebuilt.

### Hot Loop
The per-tick pipeline:
1. Receive patch
2. Build bounded prefetch
3. Run pure rules
4. Apply derived writes
5. Commit tick

Only this loop must be extremely fast.

---

## 4. Spatial Truth (v1)

**Authoritative spatial representation**:

```rust
Transform2D {
  x: i32,
  y: i32,
  facing: u8
}
```

Why:
- cheap
- deterministic
- projects cleanly to VTT and basic 3D
- avoids early physics/float complexity

---

## 5. Authoritative Component Set (v1)

Minimal but sufficient set:

1. **Identity**
   - name_id: StringId
   - tags: U64List

2. **Kind**
   - kind: EntityKind

3. **Transform2D**
   - x, y, facing

4. **LocationRef**
   - location_id: EntityId

5. **Inventory**
   - items: IdList

6. **Stats**
   - hp, ac, speed

7. **RenderableRef**
   - archetype_id: u32

Rules:
- components store facts only
- no text, meshes, UI layout, or narrative

---

## 6. Keys: Hardcoded, Scoped, Fast

### ComponentKey (hardcoded enum)
Each component has a stable numeric ID.

### FieldKey (scoped per component)
Fields are identified by `u16`, interpreted **in the context of a component**.

This avoids a global field namespace explosion and keeps evolution sane.

---

## 7. Value Model (Locked v1)

```rust
enum Value {
  I64(i64),
  U64(u64),
  Bool(bool),
  Id(EntityId),
  Str(StringId),

  IdList(Vec<EntityId>),
  U64List(Vec<u64>),
}
```

### Canonicalization Rules
- IdList and U64List are **sorted ascending**
- No duplicates unless explicitly allowed
- Empty lists are valid

Lists are replaced wholesale, never mutated incrementally.

---

## 8. Turn Contract (Profile C, Minimal + Replayable)

### TurnInput

```text
schema_version: u16
world_id: [u8; 16]
tick: u64
ruleset_id: u32

ops: Vec<Op>
prefetch: Vec<PrefetchedEntity>
```

### TurnOutput

```text
schema_version: u16
world_id: [u8; 16]
tick: u64
ruleset_id: u32
input_hash: [u8; 32]

writes: Vec<Op>
```

`input_hash = blake3(canonical_encode(TurnInput))`

---

## 9. Patch Operations (Ops)

Minimal set that actually works:

1. **SetField**
2. **AddComponent**
3. **RemoveComponent**

Ops target:
```text
(entity, component, field)
```

All ops are canonically sorted.

---

## 10. Prefetch (Bounded Read Model)

Rules may only read from prefetch.

```text
PrefetchedEntity {
  id: EntityId,
  components: Vec<PrefetchedComponent>
}

PrefetchedComponent {
  component: ComponentKey,
  fields: Vec<(FieldKey, Value)>
}
```

No queries. No IO. No lookups.

---

## 11. Rule Footprint (Concept)

Profile C requires deterministic prefetch construction.

A **RuleFootprint** defines:
- which components and fields rules may read
- which edges may be followed (Inventory → Items, Location → Location entity)

World-state machine uses this footprint + patch to build prefetch.

This prevents underfetch bugs and overfetch bloat.

---

## 12. Projections (The Cornucopia)

Projections consume authoritative state + timeline.

Examples:

- **VTTSceneView**: tokens, positions, visibility
- **3DEngineSyncView**: transforms, spawn/despawn, animation state
- **NarrativeSummaryView**: human-readable story
- **CharacterGenView**: UI flow, constraints, previews
- **AuditLogView**: who changed what and why

Rules:
- projections are disposable
- projections may be cached
- projections are never authoritative

---

## 13. Determinism & Replay

Guaranteed by:
- canonical ordering
- pure rules
- explicit inputs
- input hashing
- ruleset_id pinning

This enables:
- replay
- divergence debugging
- caching
- testing with golden inputs

---

## 14. Guiding Principle (The One to Remember)

> **Store the lightest facts you can.
> Derive everything else.**

The world engine is an artery.
Projections are organs.
Do not turn the artery into a warehouse.

---

## Status

This document defines **v1 foundation**.

Next steps (separate docs):
- RuleFootprint v1 (concrete tables)
- Prefetch construction algorithm
- Projection engine examples
- Migration & schema evolution

