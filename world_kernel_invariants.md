# World Kernel Invariants

## Purpose

This document defines the **non-negotiable invariants** of the World Kernel.

If a proposed change, feature, or optimization violates any invariant in this file, it is **incorrect by definition**, regardless of convenience, performance gains elsewhere, or feature pressure.

These invariants exist to protect:
- determinism
- replayability
- auditability
- cross-system composability

This document is intentionally strict.

---

## Definition: World Kernel

The **World Kernel** is the minimal authoritative system responsible for:
- storing factual world state
- evaluating rules deterministically
- advancing the world through discrete turns or frames

The kernel does **not**:
- render
- narrate
- query external systems
- store presentation or interpretation

---

## Invariant 1: Authority Is Facts Only

The World Kernel stores **facts**, not interpretations.

Allowed:
- entity existence
- component presence
- numeric and symbolic values
- identifiers and references

Forbidden:
- narrative text
- UI layout
- animations or assets
- derived or aggregated values

If a value can be recomputed from other values, it **must not** be stored.

---

## Invariant 2: Deterministic Evaluation

Given:
- identical authoritative state
- identical TurnInput bytes
- identical ruleset version

The kernel **must** produce identical TurnOutput bytes.

Violations include:
- unseeded randomness
- time-based logic
- platform-dependent behavior
- iteration over unordered collections

Determinism is enforced at the byte level, not the conceptual level.

---

## Invariant 3: Pure Rule Execution

Rules executed by the kernel:
- are pure functions
- have no side effects outside kernel state
- may only read from prefetch
- may only write via explicit Ops

Forbidden inside rule execution:
- IO (disk, network, clock)
- global state mutation
- hidden caches
- dynamic allocation based on data-dependent branching

Rule purity is mandatory for replay and divergence analysis.

---

## Invariant 4: Explicit Inputs and Outputs

All kernel evaluation is mediated by:
- TurnInput
- TurnOutput

There are no implicit inputs.
There are no ambient reads.

If data influences evaluation, it must appear in TurnInput or prefetch.

This invariant enables:
- hashing
- replay
- auditing
- simulation testing

---

## Invariant 5: Canonical Ordering

All collections processed by the kernel must have a canonical order.

Examples:
- entity lists
- component fields
- operations

Canonical ordering rules:
- defined
- stable
- versioned

Reliance on language-default ordering is forbidden.

---

## Invariant 6: Bounded Reads

Rules may only read data explicitly provided via prefetch.

There are:
- no ad-hoc queries
- no graph walks
- no dynamic discovery

The RuleFootprint defines the maximum readable surface.

This invariant prevents:
- underfetch nondeterminism
- overfetch performance collapse
- accidental rule coupling

---

## Invariant 7: No Feedback from Projections

The kernel does not observe projections.

Projections:
- are read-only
- are disposable
- may be incorrect without harming authority

Any feedback loop from a projection into authoritative state is a bug.

---

## Invariant 8: Versioned Meaning

All kernel behavior is scoped by:
- schema_version
- ruleset_id

Rules are never assumed to be timeless.

If meaning changes, versioning changes.

This invariant enables long-lived worlds and historical replay.

---

## Invariant 9: Failure Is Explicit

Kernel failures must be:
- detectable
- reportable
- reproducible

Silent fallback behavior is forbidden.

A failed evaluation is preferable to an incorrect one.

---

## Invariant 10: Kernel Minimalism

The kernel must remain:
- small
- boring
- predictable

If a feature can live outside the kernel, it **must**.

The kernel is an artery, not an ecosystem.

---

## Final Rule

If a proposal weakens any invariant in this document, the proposal is rejected.

No exceptions.

