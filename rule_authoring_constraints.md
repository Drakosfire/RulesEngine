# Rule Authoring Constraints

## Status

**Authoritative Authoring Contract**

This document defines the **hard constraints** imposed on all rule definitions intended to run inside the World Kernel.

Any rule that violates these constraints is **invalid by definition** and must not be executed.

These constraints exist to preserve:
- determinism
- replayability
- bounded evaluation
- kernel integrity

---

## Purpose

Rules describe **how world state may change**.

They do not:
- store authority
- manage time
- resolve intent
- perform narration

This document defines what a rule **is allowed to be**, not what it should do.

---

## Definition: Rule

A **Rule** is a pure transformation that:
- reads from prefetched world facts
- evaluates conditions
- emits zero or more write operations

Rules do not apply changes directly.

---

## Rule Input Surface

Rules may only observe:

1. Prefetched components and fields
2. Explicit parameters supplied by TurnInput ops
3. Deterministic randomness provided by the kernel

Rules may not:
- query world state
- discover entities dynamically
- inspect projections

---

## Rule Output Surface

Rules may only express effects as:

- `AddComponent`
- `RemoveComponent`
- `SetField`

Rules may not:
- mutate state directly
- emit side effects
- schedule future execution

All effects must be explicit.

---

## Purity Requirements

Rules must be **pure functions**.

Forbidden behaviors include:
- IO of any kind
- access to system time
- mutation of global state
- reliance on evaluation order

Rules must produce identical outputs given identical inputs.

---

## Deterministic Randomness

If a rule requires randomness:

- randomness must be requested explicitly
- all randomness must be seed-derived
- random draws must be ordered and count-stable

Rules may not:
- instantiate their own RNG
- branch on the presence or absence of randomness

---

## Bounded Reads

Rules may only read data included in Prefetch.

Implications:
- no graph traversal
- no "find all" queries
- no conditional reads

The RuleFootprint defines the maximum readable surface.

---

## No Hidden Authority

Rules must not:
- create entities implicitly
- infer missing data
- assume defaults not present in state

If data is required, it must be explicit.

---

## No Cross-Rule Coupling

Rules must not:
- depend on other rules executing first
- rely on side effects of other rules
- inspect emitted writes from other rules

Rule execution order must not matter.

---

## Error Handling

Rules may:
- reject execution
- emit zero writes

Rules may not:
- partially apply effects
- attempt recovery

All failure must be explicit and deterministic.

---

## Version Stability

Rules are scoped by:
- `ruleset_id`
- `schema_version`

A rule's meaning must not change without versioning.

---

## Authoring Implications

These constraints mean:
- rules are declarative, not procedural
- complex behavior emerges from composition
- convenience is subordinate to correctness

Rule authors must design with these limits in mind.

---

## Non-Negotiable Rule

If a rule cannot be expressed within these constraints, it does not belong in the World Kernel.

It must be implemented at a higher layer.

