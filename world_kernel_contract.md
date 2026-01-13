# World Kernel Contract

## Status

**Authoritative Contract**

This document defines the **World Kernel**: the minimal, deterministic, authoritative core of the system.

All higher-level systems (rules authoring, projections, VTTs, AI narration, tooling) **build on top of this contract** and may not violate it.

If a behavior is not defined here, it does not exist at the kernel level.

---

## Purpose

The World Kernel exists to:
- store authoritative world facts
- evaluate rules deterministically
- advance the world through discrete turns or frames
- support replay, audit, and divergence analysis

The kernel does **not** attempt to be expressive, friendly, or flexible.
It attempts to be **correct**.

---

## Core Design Principles

1. **Minimal Authority**  
   Store the lightest possible facts. Derive everything else.

2. **Determinism**  
   Identical inputs produce identical outputs, byte-for-byte.

3. **Purity**  
   Rule evaluation is a pure function of explicit inputs.

4. **Explicitness**  
   No implicit reads. No ambient state.

5. **Replayability**  
   Every world transition can be reproduced and inspected.

---

## Conceptual Model

The kernel operates as a deterministic state transition machine:

```
(World State, TurnInput) → TurnOutput → World State'
```

World State is mutated **only** by applying the operations emitted in TurnOutput.

---

## World State Model

### Entity

An **Entity** is an opaque identifier.

```
EntityId = u64
```

Entities have no intrinsic meaning.
Meaning arises solely from attached components.

---

### Component

A **Component** is a typed bundle of facts associated with an entity.

Characteristics:
- fixed schema per component type
- stores facts only
- no behavior

Examples:
- Identity
- Transform2D
- Stats
- Inventory

Components may be added or removed over time.

---

### ComponentKey

Each component type is identified by a **ComponentKey**.

Properties:
- stable numeric identifier
- hardcoded in the kernel
- versioned by schema version

This avoids runtime string lookups and enforces consistency.

---

### FieldKey

Fields within a component are identified by a **FieldKey**.

Properties:
- scoped to a component
- numeric (u16)
- interpreted only within the context of the component

There is no global field namespace.

---

## Value Model

All stored values conform to the kernel Value enum:

```
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

- Lists are sorted ascending
- Duplicate elements are forbidden unless explicitly allowed
- Empty lists are valid

All values must have a single canonical encoding.

---

## Operations (Ops)

The kernel state is modified exclusively through **Ops**.

Supported Ops:

1. `AddComponent`
2. `RemoveComponent`
3. `SetField`

Each Op targets:
```
(entity_id, component_key, field_key?)
```

Ops are:
- explicit
- atomic
- canonically ordered

There are no hidden mutations.

---

## Turn Contract

### TurnInput

```
schema_version: u16
world_id: [u8; 16]
tick: u64
ruleset_id: u32

ops: Vec<Op>
prefetch: Vec<PrefetchedEntity>
```

Properties:
- fully explicit
- self-contained
- canonicalized before hashing

---

### TurnOutput

```
schema_version: u16
world_id: [u8; 16]
tick: u64
ruleset_id: u32
input_hash: [u8; 32]

writes: Vec<Op>
```

`input_hash = hash(canonical_encode(TurnInput))`

TurnOutput is the **only** way the kernel expresses state change.

---

## Prefetch Model

Rules may only read from prefetched data.

```
PrefetchedEntity {
  id: EntityId,
  components: Vec<PrefetchedComponent>
}

PrefetchedComponent {
  component: ComponentKey,
  fields: Vec<(FieldKey, Value)>
}
```

Prefetch:
- is bounded
- is deterministic
- contains no queries

---

## Rule Evaluation Boundary

During evaluation, rules:
- read only from prefetch
- write only via emitted Ops
- cannot allocate new authority

The kernel does not introspect rules.
It enforces their boundaries.

---

## Determinism Guarantees

The kernel guarantees:
- canonical ordering of all inputs and outputs
- stable hashing
- platform-independent behavior

Violations of determinism are kernel bugs.

---

## Versioning

Meaning is scoped by:
- `schema_version`
- `ruleset_id`

Historical turns are evaluated under the ruleset active at that time.

---

## Non-Responsibilities

The World Kernel does not:
- render
- narrate
- resolve UI state
- perform IO
- interpret intent
- infer meaning

Those responsibilities belong to higher layers.

---

## Contract Stability

This contract is intended to be:
- stable
- conservative
- difficult to change

Breaking changes require a new major version.

---

## Guiding Rule

> Store the lightest facts you can.
> Derive everything else.

The kernel is an artery.
Do not turn it into a warehouse.

