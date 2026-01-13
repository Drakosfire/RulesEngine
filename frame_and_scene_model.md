# Frame and Scene Model

## Status

**Authoritative Temporal Model**

This document defines how **time, turns, and scenes** are represented and advanced within the World Kernel.

It specifies when rules execute, how state progresses, and how complex TTRPG play is decomposed into deterministic steps.

---

## Purpose

The Frame and Scene Model exists to:
- impose a deterministic notion of time
- support turn-based and phase-based play
- enable replay, rollback, and inspection
- decouple narrative pacing from mechanical progression

Time in the kernel is mechanical, not experiential.

---

## Core Concepts

### Frame

A **Frame** is the smallest unit of world advancement.

A frame:
- corresponds to a single kernel evaluation
- consumes one TurnInput
- produces one TurnOutput
- advances the world from state N to state N+1

Frames are:
- discrete
- ordered
- immutable once committed

---

### Tick

A **Tick** is the global index of frames.

Properties:
- monotonically increasing
- unique per frame
- included in TurnInput and TurnOutput

Ticks define absolute mechanical time.

---

### Turn

A **Turn** is a semantic grouping of one or more frames.

Examples:
- a character taking an action
- a combat round
- an environmental resolution step

Turns do not exist at the kernel level as stored state.
They are derived concepts built atop frames.

---

### Scene

A **Scene** is a higher-level construct representing a coherent span of play.

Examples:
- combat encounter
- social exchange
- exploration phase

Scenes:
- are not authoritative kernel entities
- may be represented as projections or metadata
- may span many frames

The kernel is scene-agnostic.

---

## Temporal Hierarchy

```
Scene
  └── Turn (derived)
        └── Frame (authoritative)
```

Only Frames are authoritative.

---

## Frame Lifecycle

Each frame follows the Rule Evaluation Pipeline:

1. Receive TurnInput
2. Canonicalize
3. Construct Prefetch
4. Evaluate Rules
5. Collect Writes
6. Validate and Order
7. Emit TurnOutput
8. Commit

Once committed, a frame is immutable.

---

## Sequencing and Order

Frames are evaluated strictly in tick order.

Rules:
- no frame may observe future frames
- no frame may modify past frames
- no concurrent frame evaluation

Parallelism may exist outside the kernel but must serialize at commit.

---

## Choice and Intent

Player or GM intent is not resolved inside the kernel.

Instead:
- intent is expressed as TurnInput ops
- validation occurs via rules
- invalid intent results in failed frames

The kernel does not infer intent.

---

## Randomness and Time

Randomness is scoped to frames.

Rules:
- random draws are deterministic per frame
- RNG state advances in a defined order
- replay reproduces identical draws

Randomness does not advance time.

---

## Multi-Phase Actions

Complex actions may require multiple frames.

Examples:
- casting a spell with wind-up and resolution
- multi-step environmental effects

Such actions are modeled as:
- explicit state transitions across frames
- never as hidden delays or timers

---

## Interrupts and Reactions

Interrupts and reactions are modeled as:
- additional frames
- explicit ops
- rule-mediated legality

There is no concept of "mid-frame" execution.

---

## Scene Transitions

Scene transitions:
- do not alter kernel semantics
- may be represented as metadata
- may trigger projection rebuilds

The kernel does not track narrative boundaries.

---

## Replay and Rollback

Because frames are immutable:
- replay is a linear re-evaluation of frames
- rollback is truncation and re-simulation

Scene context is reconstructed from projections.

---

## Failure Handling

If a frame fails:
- no state is committed
- tick does not advance
- error is reported

There is no partial frame.

---

## Non-Goals

This model does not:
- encode initiative rules
- encode turn order
- encode narrative pacing

Those are rule- or projection-level concerns.

---

## Summary

The Frame and Scene Model ensures that:
- all world change is discrete and inspectable
- time is deterministic
- complex play emerges from simple steps

Scenes tell stories.
Frames change facts.

