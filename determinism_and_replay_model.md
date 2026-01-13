# Determinism and Replay Model

## Status

**System-Wide Invariant**

This document defines the guarantees, requirements, and failure modes related to **determinism and replay** across the entire system.

Determinism is not an optimization.
Replay is not a debugging convenience.

They are foundational properties.

---

## Purpose

The Determinism and Replay Model exists to:
- guarantee reproducible world evolution
- enable audit and inspection
- support rollback and branching simulation
- establish trust in automated rule execution

Any system that weakens determinism weakens the engine itself.

---

## Definition: Determinism

Determinism means:

> Given identical initial state and identical inputs, the system produces identical outputs, byte-for-byte.

This applies to:
- rule evaluation
- state transitions
- emitted operations

Determinism is evaluated at the **serialization boundary**, not the conceptual boundary.

---

## Scope of Determinism

Determinism applies to:
- the World Kernel
- rule execution
- TurnInput → TurnOutput mapping

Determinism does **not** apply to:
- projections
- UI rendering
- narrative generation

Authority is deterministic. Interpretation may not be.

---

## Replay Model

Replay is defined as:

> Re-evaluating a sequence of frames to reproduce historical world state.

Replay requires:
- initial world snapshot
- ordered frame history
- original ruleset versions

Replay must reproduce:
- identical TurnOutputs
- identical world states

---

## Frame Immutability

Once a frame is committed:
- it is immutable
- it is never reinterpreted
- it is never partially applied

All replay operates on committed frames only.

---

## Input Completeness

Replay correctness requires:
- all inputs to evaluation are explicit
- no ambient state
- no hidden dependencies

If a value influences evaluation, it must appear in TurnInput or Prefetch.

---

## Randomness Model

Randomness is:
- deterministic
- seed-derived
- frame-scoped

Rules:
- RNG seeds are explicit
- RNG consumption order is stable
- RNG usage is count-stable

Randomness must replay exactly.

---

## Canonical Encoding

All replay-relevant data must have:
- canonical encoding
- stable ordering
- versioned schemas

Non-canonical data invalidates replay.

---

## Divergence Detection

Divergence is detected by:
- comparing TurnOutput hashes
- validating expected state transitions

Divergence indicates:
- a kernel bug
- a rule violation
- a version mismatch

Silent divergence is forbidden.

---

## Rollback Model

Rollback is implemented as:
- truncation of frame history
- replay from a prior snapshot

Rollback does not mutate history.
It replaces it.

---

## Branching Timelines

The system may support branching by:
- forking frame history
- replaying from a shared prefix

Branches are independent worlds.

---

## Observability and Debugging

Replay enables:
- step-by-step inspection
- causal tracing
- test reproduction

Debug metadata:
- must not affect authority
- must not alter replay

---

## Failure Modes

Replay failure may result from:
- schema mismatch
- ruleset drift
- non-deterministic code

Replay failure is a system defect.

---

## Non-Goals

This model does not:
- guarantee identical visual output
- synchronize client clocks
- infer missing history

Replay requires complete data.

---

## Summary

Determinism enables replay.
Replay enables trust.

If replay cannot be trusted, nothing built on the system can be.

