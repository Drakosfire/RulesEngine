# Standard Projections Catalog

## Status

**Canonical Projection Vocabulary**

This document enumerates the **standard projections** recognized by the system.

These projections are not mandatory implementations, but **stable conceptual contracts** that integrations may rely on.

Each projection defined here:
- obeys the Projection Contract
- is read-only
- is disposable
- has no authority

---

## Purpose

The Standard Projections Catalog exists to:
- establish a shared vocabulary
- reduce integration ambiguity
- enable parallel development
- prevent projection sprawl

This document defines *what a projection means*, not how it is rendered.

---

## Projection Definition Template

Each standard projection specifies:

- **Name**
- **Intent**
- **Inputs Required**
- **Outputs Expected**
- **Non-Goals**

No projection may exceed this scope.

---

## 1. VTTSceneView

### Intent

Provide a spatial and interactive representation of the world suitable for a Virtual Tabletop.

### Inputs Required

- world state (entities, components)
- Transform components
- visibility-related facts
- frame history (optional, for animation)

### Outputs Expected

- token positions
- layer/group structure
- visibility and targeting affordances

### Non-Goals

- authoritative collision
- physics simulation
- rules enforcement

---

## 2. CharacterGenView

### Intent

Present character creation and advancement options based on rules and constraints.

### Inputs Required

- entity components
- ruleset definitions
- frame history (optional)

### Outputs Expected

- legal choices
- previews of derived stats
- validation feedback

### Non-Goals

- committing changes
- storing character state
- resolving randomness

---

## 3. CharacterSheetView

### Intent

Summarize the current mechanical state of a character for human consumption.

### Inputs Required

- entity components
- derived values from rules

### Outputs Expected

- stats
- resources
- equipment summaries

### Non-Goals

- authoritative calculations
- persistent caching
- narrative embellishment

---

## 4. NarrativeSummaryView

### Intent

Generate a human-readable narrative interpretation of frame history.

### Inputs Required

- frame history
- world state snapshots

### Outputs Expected

- textual summaries
- event descriptions

### Non-Goals

- canonical storytelling
- rule interpretation
- altering world state

---

## 5. AuditLogView

### Intent

Expose a precise, inspectable record of world changes.

### Inputs Required

- TurnOutputs
- frame metadata

### Outputs Expected

- ordered change logs
- causality traces
- provenance markers

### Non-Goals

- summarization
- redaction
- visualization

---

## 6. ReplayView

### Intent

Enable deterministic replay of past frames for debugging or analysis.

### Inputs Required

- initial world state
- frame history

### Outputs Expected

- reconstructed states
- step-by-step playback

### Non-Goals

- real-time interaction
- authority mutation

---

## 7. SimulationAnalysisView

### Intent

Support analytical tooling such as balance analysis, AI training, or testing.

### Inputs Required

- world state
- rule definitions
- frame history

### Outputs Expected

- metrics
- distributions
- simulation results

### Non-Goals

- player-facing presentation
- narrative output

---

## Projection Extension Rules

New projections may be introduced if they:
- obey the Projection Contract
- declare clear intent
- do not overlap authority

Projections that redefine kernel concepts are forbidden.

---

## Summary

Standard projections:
- provide shared meaning
- enable interoperability
- constrain interpretation

They make the system legible without weakening the core.
