# DungeonMind RuleEngine

**DungeonMind RuleEngine** is a deterministic, replayable rules and world-state engine designed for turn-based tabletop role‑playing games (TTRPGs).

It is not a Virtual Tabletop.
It is not a narrative engine.
It is not a convenience framework.

It is a **kernel**: a minimal, authoritative system for evolving world state under formalized rules, with strong guarantees around determinism, replay, and auditability.

---

## What This Is

DungeonMind RuleEngine provides:

- A **minimal authoritative world model** (facts only)
- A **deterministic rule evaluation pipeline**
- A **frame-based time model** suitable for turn-based play
- **Replay, rollback, and divergence detection** as first‑class capabilities
- A strict separation between **authority** and **interpretation**

Everything else (UI, VTTs, AI narration, analytics, testing tools) lives **outside** the kernel and integrates through explicit surfaces.

---

## What This Is Not

DungeonMind RuleEngine explicitly does **not**:

- Render maps or tokens
- Manage chat, players, or sessions
- Interpret natural language rules
- Decide story outcomes or narrative pacing
- Optimize for convenience over correctness

If you are looking for a VTT or a content generator, this is not that.

---

## Core Design Principles

1. **Authority Is Facts Only**  
   Store the lightest possible facts. Derive everything else.

2. **Determinism Is Mandatory**  
   Identical inputs must produce identical outputs, byte‑for‑byte.

3. **Rules Are Pure**  
   Rule execution is a pure function of explicit inputs.

4. **Time Is Discrete**  
   World evolution happens in immutable frames.

5. **Replay Is Trust**  
   If it cannot be replayed, it cannot be trusted.

---

## High‑Level Architecture

```
External Systems (VTTs, AI, Tools)
            ↓
     Integration Surfaces
            ↓
   ┌─────────────────────┐
   │ DungeonMind Kernel  │
   │  • World State      │
   │  • Rule Evaluation  │
   │  • Frame History    │
   └─────────────────────┘
            ↓
        Projections
 (VTT views, narrative, logs)
```

The kernel is authoritative.
Everything else observes.

---

## Repository Structure

```text
world-engine/
├── kernel/
│   ├── WORLD_KERNEL_CONTRACT.md
│   ├── WORLD_KERNEL_INVARIANTS.md
│
├── rules/
│   ├── RULE_EVALUATION_PIPELINE.md
│   ├── RULE_AUTHORING_CONSTRAINTS.md
│   └── FRAME_AND_SCENE_MODEL.md
│
├── projections/
│   ├── PROJECTION_CONTRACT.md
│   └── STANDARD_PROJECTIONS_CATALOG.md
│
├── integration/
│   ├── INTEGRATION_SURFACES.md
│   └── RUST_RUNTIME_PROFILE.md
│
└── system/
    ├── DETERMINISM_AND_REPLAY_MODEL.md
    └── NON_GOALS_AND_FORBIDDEN_DESIGNS.md
```

This repository is a **systems specification**, not an application.

---

## Intended Use Cases

DungeonMind RuleEngine is designed for:

- Deterministic TTRPG simulation
- Rules engines backing VTTs
- AI‑assisted GM tools that must remain auditable
- Automated testing and balance analysis
- Replayable campaign state tracking

It is especially suited for:
- turn‑based combat
- phase‑based encounters
- structured exploration
- rules‑heavy or homebrew systems

---

## Language and Implementation

The reference implementation targets **Rust**.

Rust is used as an enforcement tool to:
- prevent accidental nondeterminism
- constrain memory and mutation
- protect the hot evaluation loop

Rust is a means, not an ideology.

---

## Project Philosophy

DungeonMind RuleEngine is intentionally:
- strict
- opinionated
- limited

These limits are what make it reliable.

The engine would rather reject a feature than accept a lie.

---

## Status

This repository currently contains:
- a complete architectural specification
- system contracts and invariants

Implementation is expected to follow these documents exactly.

Breaking changes require versioning.

---

## Guiding Rule

> Store the lightest facts you can.  
> Derive everything else.

If a proposal weakens determinism, replay, or authority boundaries, it does not belong here.

---

## License

TBD

