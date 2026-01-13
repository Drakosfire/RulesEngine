# Projection Contract

## Status

**Authoritative Read-Only Contract**

This document defines what a **Projection** is, how it may be constructed, and the strict boundaries between projections and the World Kernel.

Any system that reads from the kernel without mutating it operates under this contract.

---

## Purpose

Projections exist to:
- interpret authoritative world state
- present information to humans or tools
- derive views optimized for specific use cases

Projections do **not**:
- own authority
- change world state
- influence rule execution

They are observers, not participants.

---

## Definition: Projection

A **Projection** is a deterministic or non-deterministic function of:
- authoritative world state
- committed frame history
- optional configuration parameters

Formally:
```
Projection = f(WorldState, FrameHistory, Config) -> View
```

The output is disposable.

---

## Authority Boundary

The World Kernel is authoritative.

Projections:
- must never mutate kernel state
- must never influence kernel execution
- must never be consulted during rule evaluation

Any feedback path from projection to kernel is forbidden.

---

## Read Model Guarantees

Projections may read:
- committed world state
- committed TurnOutputs
- frame metadata

Projections may not read:
- in-flight evaluation state
- partial frames
- uncommitted writes

The read surface is stable and versioned.

---

## Determinism Expectations

Projections may be:
- deterministic
- non-deterministic

However:
- non-determinism must not affect authority
- projection output must never imply untrue state

If a projection lies, it must be obviously cosmetic.

---

## Disposability

All projections are disposable.

Rules:
- projections may be deleted at any time
- projections must be rebuildable from authority
- cached projections are advisory only

Loss of a projection must never corrupt the world.

---

## Cache Semantics

Projections may:
- cache derived data
- memoize expensive computations

Caches must:
- be invalidatable
- be rebuildable
- never become authoritative

Cache corruption must be survivable.

---

## Temporal Interpretation

Projections may:
- summarize frame ranges
- group frames into turns or scenes
- interpret temporal patterns

They must not:
- reorder frames
- omit committed frames
- invent transitions

Time is defined by ticks, not projections.

---

## Error Handling

Projection failures:
- must not halt kernel operation
- must not affect authority
- may degrade user experience only

Projections fail soft.

---

## Versioning

Projections must declare:
- supported schema versions
- supported ruleset versions

Unsupported versions must fail explicitly.

---

## Security and Trust

Projections are untrusted relative to the kernel.

The kernel must:
- assume projections may be buggy
- assume projections may be hostile

Authority is protected by isolation.

---

## Non-Goals

This contract does not:
- define specific projections
- mandate projection architectures
- prescribe UI patterns

Those belong in separate documents.

---

## Summary

Projections exist to observe, interpret, and present.

They are:
- read-only
- disposable
- isolated

The kernel tells the truth.
Everything else listens.

