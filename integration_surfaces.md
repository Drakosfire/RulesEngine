# Integration Surfaces

## Status

**Authoritative Boundary Definition**

This document defines the **explicit integration surfaces** through which external systems may interact with the World Engine.

Any interaction not defined here is **unsupported by design**.

---

## Purpose

Integration Surfaces exist to:
- allow external systems to observe and influence the world
- preserve kernel invariants under integration pressure
- enable modular tooling, UIs, and AI systems

They exist to constrain, not to empower.

---

## Integration Principles

All integrations must respect the following principles:

1. **Kernel Authority**  
   The kernel is the sole authority over world state.

2. **Explicit Boundaries**  
   All interactions cross named surfaces.

3. **Asymmetric Trust**  
   External systems are untrusted by default.

4. **Replay Preservation**  
   Integration must not compromise determinism or replay.

---

## Surface Categories

Integration surfaces are grouped by **direction of influence**:

1. Input Surfaces (External → Kernel)
2. Output Surfaces (Kernel → External)
3. Tooling Surfaces (Kernel ↔ Tools)

---

## Input Surfaces

### 1. Turn Submission Surface

**Purpose:**
Submit player, GM, AI, or system intent to the kernel.

**Mechanism:**
- construction of TurnInput
- submission for evaluation

**Constraints:**
- all intent must be explicit
- no partial submission
- invalid intent must fail cleanly

The kernel does not infer meaning.

---

### 2. Ruleset Selection Surface

**Purpose:**
Select which ruleset governs evaluation.

**Mechanism:**
- ruleset_id binding

**Constraints:**
- ruleset must be immutable for the turn
- ruleset changes require explicit transition

---

## Output Surfaces

### 3. TurnOutput Emission Surface

**Purpose:**
Expose authoritative results of evaluation.

**Mechanism:**
- serialized TurnOutput

**Constraints:**
- output is append-only
- consumers must treat output as read-only

---

### 4. Projection Read Surface

**Purpose:**
Allow external systems to observe world state.

**Mechanism:**
- projection construction
- read-only access

**Constraints:**
- projections must obey Projection Contract
- kernel state is never directly exposed

---

## Tooling Surfaces

### 5. Replay and Inspection Surface

**Purpose:**
Enable debugging, auditing, and simulation analysis.

**Mechanism:**
- frame history access
- replay execution

**Constraints:**
- tooling may not mutate authority
- replay must preserve determinism

---

### 6. Testing and Validation Surface

**Purpose:**
Support automated testing and verification.

**Mechanism:**
- deterministic test harnesses
- golden input/output pairs

**Constraints:**
- tests must not rely on projection behavior

---

## Asynchronous Integrations

External systems may operate asynchronously.

Rules:
- kernel evaluation is synchronous
- async systems must serialize at TurnInput submission

No async influence during evaluation is permitted.

---

## AI System Integration

AI systems:
- may propose TurnInputs
- may consume projections
- may generate narrative

AI systems may not:
- bypass rules
- mutate authority
- observe in-flight evaluation

AI is advisory unless explicitly authorized.

---

## Failure Isolation

Integration failures:
- must not crash the kernel
- must not corrupt authority
- must be detectable

Kernel failure is preferable to silent corruption.

---

## Security Posture

The kernel assumes:
- integrations may be buggy
- integrations may be malicious

All surfaces must be defensively designed.

---

## Non-Goals

This document does not:
- prescribe transport protocols
- mandate APIs
- define authentication schemes

Those are deployment concerns.

---

## Summary

Integration Surfaces define how the world engine touches the outside world.

They exist to:
- protect authority
- preserve determinism
- enable safe extensibility

If an integration needs more power, the design is wrong.

