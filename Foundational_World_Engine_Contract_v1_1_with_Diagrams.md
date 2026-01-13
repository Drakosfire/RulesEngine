# Foundational World Engine Contract v1.1
## Rules Toolchain, Utilities, and Frame-Based Scene Simulation

> **Status:** Expansion Document  
> **Builds on:** Foundational World Engine Contract v1  
> **Purpose:** Combine world-state determinism with rules-driven scene simulation

---

## 1. Core Mental Model

```
Frame N
  ↓ (Evaluate Rules + Constraints)
EvaluationResult
  ↓ (Choices / Dice / Decisions)
TransitionCommit
  ↓
Frame N+1
```

---

## 2. Project-Level Architecture (System Boundaries)

```
┌──────────────────────────────┐
│  Project A: Rule Ingestion   │
│  (AI, PDFs, Text)            │
└─────────────┬────────────────┘
              ▼
┌──────────────────────────────┐
│ Project B: Rule Repository   │
│ (Canonical Schemas)          │
└─────────────┬────────────────┘
              ▼
┌──────────────────────────────┐
│ Project C: Rule Compiler     │
│ (Deterministic Build Step)   │
└─────────────┬────────────────┘
              ▼
┌──────────────────────────────┐
│ Project D: Runtime Engine    │
│ (Pure Evaluation)            │
└─────────────┬────────────────┘
              ▼
┌──────────────────────────────┐
│ Project E: Choice Systems    │
│ (Players / GM / AI / RNG)    │
└─────────────┬────────────────┘
              ▼
┌──────────────────────────────┐
│ Project F: Frame Store       │
│ (Timeline / Replay)          │
└──────────────────────────────┘
```

---

## 3. Utilities and Constraints

```
Utility
 ├─ Target Selector
 ├─ Effects (Patch Templates)
 ├─ Costs (Resources)
 ├─ Preconditions
 └─ Constraints
```

---

## 4. Runtime Engine Internal Flow

```
Frame N
  ↓
Fact Derivation
  ↓
Utility Candidate Selection
  ↓
Constraint Evaluation
  ↓
Symbolic Effect Application
  ↓
EvaluationResult
```

---

## 5. Projections

```
World State
  ↓
Projections
  ├─ VTT Scene View
  ├─ Character Sheet View
  ├─ Narrative Summary
  └─ Audit Log
```

---

## 6. One-Sentence Summary

> A deterministic, replayable world engine where rules define constrained utilities and scenes evolve as immutable frames.
