# Rule Evaluation Pipeline

## Status

**Authoritative Runtime Description**

This document defines **how rules are evaluated** inside the World Kernel.

It does not define rule semantics or content.
It defines **execution order, boundaries, and guarantees**.

Any runtime behavior not described here is invalid.

---

## Purpose

The Rule Evaluation Pipeline exists to:
- enforce kernel invariants
- ensure deterministic rule execution
- provide a stable execution model for all rulesets
- make replay, debugging, and divergence analysis possible

This pipeline is intentionally rigid.

---

## High-Level Overview

Each turn is evaluated as a **single, deterministic transaction**.

```
TurnInput
  ↓
Canonicalization
  ↓
Prefetch Construction
  ↓
Rule Evaluation
  ↓
Write Collection
  ↓
Validation & Ordering
  ↓
TurnOutput
```

No stage may be skipped or reordered.

---

## Stage 0: Turn Boundary

The kernel evaluates exactly **one turn** at a time.

A turn is defined by:
- a specific `tick`
- a specific `ruleset_id`
- a closed set of input ops

There is no partial turn evaluation.

---

## Stage 1: Canonicalization

All inputs are canonicalized before evaluation.

This includes:
- sorting ops
- canonical encoding of values
- normalization of lists

Canonicalization guarantees:
- stable hashing
- deterministic iteration

Failure to canonicalize is a kernel bug.

---

## Stage 2: Prefetch Construction

The kernel constructs a **bounded read model** for rule evaluation.

Inputs:
- current authoritative world state
- TurnInput.ops
- RuleFootprint for the active ruleset

Outputs:
- a fixed Prefetch structure

Rules:
- no dynamic queries
- no conditional expansion
- no runtime graph traversal

Prefetch contents are complete and final.

---

## Stage 3: Rule Evaluation

Rules are evaluated against the Prefetch.

Properties:
- pure function execution
- no side effects
- no mutation of authoritative state

Rules may:
- read prefetched values
- compute intermediate values
- emit symbolic write intents

Rules may not:
- observe world state outside prefetch
- perform IO
- allocate authority

---

## Stage 4: Write Collection

Rules emit **write intents** as Ops.

Characteristics:
- symbolic
- explicit
- side-effect free

Writes are collected into an unordered buffer.

Rules do not apply writes directly.

---

## Stage 5: Validation & Ordering

Collected writes are:
- validated against schema
- checked for invariant violations
- canonically ordered

Invalid writes cause the turn to fail.

Partial application is forbidden.

---

## Stage 6: TurnOutput Construction

The kernel constructs TurnOutput.

Includes:
- canonical write list
- input hash
- version identifiers

TurnOutput is the sole artifact of evaluation.

---

## Failure Modes

A turn may fail due to:
- invalid ops
- schema violations
- invariant violations
- rule execution errors

Failures are:
- explicit
- reproducible
- non-destructive

No state is mutated on failure.

---

## Determinism Guarantees

The pipeline guarantees:
- identical inputs → identical outputs
- stable ordering
- replayable evaluation

Any nondeterminism is a defect.

---

## Observability

The kernel may emit:
- evaluation traces
- debug metadata

These artifacts are:
- non-authoritative
- excluded from TurnOutput
- discardable

---

## Non-Goals

This pipeline does not:
- define rule syntax
- define rule authoring tools
- resolve player intent
- generate narrative

Those concerns belong elsewhere.

---

## Summary

The Rule Evaluation Pipeline is the **spine of the system**.

It ensures that:
- rules are safe to run
- state transitions are auditable
- higher-level systems can trust the kernel

If this pipeline is compromised, the system collapses.

