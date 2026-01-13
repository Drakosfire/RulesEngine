# Rust Runtime Profile

## Status

**Implementation Constraint Profile**

This document defines the runtime and implementation constraints for the World Engine when implemented in Rust.

Rust is used here as an enforcement mechanism for:
- determinism
- safety
- performance predictability

This is not a general Rust style guide.

---

## Purpose

The Rust Runtime Profile exists to:
- protect the hot loop
- prevent accidental nondeterminism
- constrain allocations and data structures
- standardize performance expectations

Any implementation choice that violates these constraints is incorrect.

---

## Build Profile Assumptions

The engine must be valid under:
- debug builds (correctness)
- release builds (performance)

Behavior must not differ between profiles except for:
- logging
- debug assertions

No logic may depend on compilation mode.

---

## Determinism Constraints

### Forbidden Sources of Nondeterminism

Implementation must not depend on:
- wall clock time
- thread scheduling
- hash iteration order
- floating point nondeterminism
- platform-specific integer overflow behavior

Rules:
- prefer integer math in the kernel
- avoid float use in authoritative state

---

### HashMap and Ordering

Rust standard `HashMap` iteration order is not stable.

Rules:
- do not iterate `HashMap` when order matters
- prefer ordered structures for canonical output
- if hashing is required, use stable ordering before output

The kernel must never produce output dependent on hash iteration order.

---

## Hot Loop Performance Profile

The per-frame evaluation path must be:
- bounded
- predictable
- allocation-aware

### Allocation Rules

Within rule evaluation:
- avoid heap allocations on critical paths
- preallocate buffers where possible
- reuse memory across frames when safe

Allocations are not forbidden, but must be measured and bounded.

---

### Data Layout

Prefer:
- compact structs
- enums with explicit variants
- contiguous storage for frequently iterated collections

Avoid:
- deep pointer-chasing graphs
- hidden allocations

The kernel should behave like a data-oriented engine.

---

## Concurrency Profile

Kernel evaluation is single-threaded at the logical level.

Rules:
- no concurrent mutation of authoritative state
- no shared mutable global state

Parallelism may exist:
- outside the kernel
- in projection computation
- in tooling

Evaluation and commit must serialize.

---

## Error Handling

Rules:
- do not panic on malformed user input
- treat invalid TurnInput as a recoverable error
- use explicit error types

Panics are reserved for:
- internal invariants
- impossible states

A crash is always worse than a clean failure.

---

## Canonical Encoding

All serialization used for:
- TurnInput
- TurnOutput
- hashing

must be:
- canonical
- stable
- versioned

Avoid:
- serde default output without canonicalization
- platform-dependent encoding

Canonical encoding must be explicitly defined.

---

## Memory Safety and Ownership

Rust ownership is used to:
- eliminate aliasing bugs
- prevent accidental shared mutation

Rules:
- authoritative state mutation occurs only at commit
- evaluation uses immutable views

If borrowing rules force awkward design, prefer correctness over cleverness.

---

## Dependency Policy

Dependencies must be treated as part of the determinism surface.

Rules:
- minimize dependencies in kernel
- avoid libraries with hidden global state
- avoid libraries that use randomness internally
- pin versions explicitly

Upgrading dependencies is a versioned event.

---

## Testing Expectations

The Rust implementation must include:
- golden input/output tests
- replay equivalence tests
- divergence detection tests

Tests must:
- run without network
- run without time dependence

Determinism is proven by tests, not assumed.

---

## Non-Goals

This profile does not:
- prescribe a module layout
- mandate async runtimes
- define API transport

It defines constraints for correctness and performance.

---

## Summary

Rust is used to make the kernel:
- safe
- fast
- predictable

If Rust makes it hard to violate invariants, it is doing its job.
