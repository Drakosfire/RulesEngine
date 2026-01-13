# Non-Goals and Forbidden Designs

## Status

**Architectural Boundary Declaration**

This document defines what the World Engine explicitly **does not attempt to do**, and which design approaches are **forbidden by construction**.

These are not missing features.
They are intentional absences.

---

## Purpose

This document exists to:
- prevent architectural drift
- block well-intentioned but corrosive ideas
- preserve long-term coherence
- make refusal explicit and principled

Every mature system is defined as much by what it rejects as by what it builds.

---

## Non-Goals (Explicitly Out of Scope)

### 1. Being a Virtual Tabletop

The World Engine is not:
- a UI framework
- a map renderer
- a chat system
- a real-time collaboration tool

Those systems may integrate with the engine, but they do not belong inside it.

---

### 2. Being a Narrative Authority

The engine does not:
- decide story outcomes
- enforce narrative structure
- determine pacing or drama

Narrative is a projection.
The engine only changes facts.

---

### 3. Being a Rules Interpreter for Prose

The engine does not execute:
- natural language rules
- ambiguous text
- editorial intent

Rules must be formalized before they reach the kernel.

---

### 4. Solving All TTRPG Systems

The engine does not:
- claim to support every RPG mechanic
- promise universal expressiveness

Some systems may not fit.
That is acceptable.

---

### 5. Optimizing for Convenience

Convenience is subordinate to:
- correctness
- determinism
- replayability

If a feature makes the system easier but less trustworthy, it is rejected.

---

## Forbidden Designs (Hard No)

### 6. God Objects

Forbidden:
- central managers with implicit authority
- global state containers
- objects that "know everything"

Authority must be explicit and local.

---

### 7. Hidden Queries

Forbidden:
- rule-side queries into world state
- dynamic entity discovery
- "just look it up" helpers

All reads must be bounded and explicit.

---

### 8. Implicit State Mutation

Forbidden:
- side effects during evaluation
- background updates
- lazy writes

All mutation occurs via explicit Ops.

---

### 9. Mid-Evaluation IO

Forbidden:
- database access
- network calls
- clock reads
- filesystem interaction

Evaluation must be pure.

---

### 10. Rule Ordering Dependencies

Forbidden:
- rules that rely on execution order
- rules that inspect other rule outputs
- phase-specific hacks

Rule composition must be order-independent.

---

### 11. Projection Feedback Loops

Forbidden:
- UI-driven rule shortcuts
- narrative-driven state mutation
- AI-generated authority

Projections observe.
They do not decide.

---

### 12. Soft Determinism

Forbidden:
- "mostly deterministic" logic
- best-effort replay
- tolerance for divergence

Determinism is binary.

---

### 13. Silent Failure

Forbidden:
- swallowed errors
- fallback behavior that hides bugs
- partial state application

Failure must be loud and reproducible.

---

### 14. Schema Drift Without Versioning

Forbidden:
- changing meaning without version bumps
- retroactive rule reinterpretation

History must remain interpretable.

---

## Design Smells (Early Warning Signs)

The following indicate likely violation:

- "We can just store it for convenience"
- "The UI needs to know this"
- "Let the rule figure it out dynamically"
- "We can fix it in the projection"
- "It probably won’t diverge"

These are stop-the-line moments.

---

## When to Say No

Say no when:
- a feature weakens replay
- a shortcut hides authority
- a change blurs boundaries

Saying no preserves the system.

---

## Summary

This engine is:
- strict
- opinionated
- limited by design

Those limits are what make it reliable.

If a proposal conflicts with this document, the proposal is rejected.

No exceptions.
