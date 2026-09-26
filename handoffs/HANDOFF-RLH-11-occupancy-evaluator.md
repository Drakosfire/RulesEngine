# HANDOFF — RLH-11 occupancy evaluator

**Status:** ACTIVE — implementation candidate, pending review  
**Repository:** `Drakosfire/RulesEngine`  
**Authority:** `Drakosfire/DungeonOverMind/Docs/Plans/PLAN-rules-lawyer-graph-experiment.md`  
**Predecessor:** `RLH_10_RULE_ARTIFACT_CONTRACT_ACCEPTED`  
**Primary question:** Can the smallest deterministic evaluator consume the reviewed occupancy artifact and produce a replay-stable placement decision without natural-language interpretation or IO?  
**Final Phase H research witness**

## Scope

Implement only enough runtime to prove the existing occupancy vertical slice.

Input explicitly provides:

- ruleset;
- actor/entity traits and size;
- occupied-cell entity;
- relation state;
- ally-prone condition;
- end-move intent.

Output contains:

- accept/reject;
- no state mutation for this legality check;
- applied rule ID(s);
- deterministic violation/reason code when rejected;
- audit trace to formal artifact/provenance identity.

## Runtime rules

- validate artifact before evaluation;
- compile/normalize into immutable runtime form if needed;
- evaluation is pure over artifact + explicit request/prefetch;
- no DB, filesystem, network, clock, Jev, LLM, or DungeonMind calls during evaluation;
- no dynamic rule discovery or graph traversal;
- stable canonical ordering and hashes.

## Cases

1. ordinary Medium vs occupied Medium → reject;
2. ally + Prone occupant → reject (the sourced rule has no ally-Prone allowance);
3. ally without allowance → reject;
4. hostile/neutral occupied cell → reject;
5. missing required context → explicit failure, never guessed default;
6. unsupported artifact/schema → fail before evaluation;
7. identical inputs replay with identical semantic output/digest.

## Implementation language

Honor the repository's accepted Rust target if current authority still says Rust when activated. Do not choose another language without an explicit architecture decision.

Bootstrap only the minimum crate/modules/test harness needed for this proof. Do not build a VTT, ECS, scheduler, persistence layer, or generalized DSL beyond forms required by RLH-10.

## Acceptance witness

```text
exact source EvidenceUnits
→ reviewed RLH-09 artifact
→ accepted RLH-10 contract
→ deterministic occupancy request
→ deterministic placement decision
→ audit trace to artifact/evidence identity
```

Acceptance token:

```text
RLH_11_OCCUPANCY_EVALUATOR_ACCEPTED
```

This proves one formalization path, not an entire ruleset.

The historical ally-Prone case was superseded by the user's sourced-rule correction and RLH-09. The evaluator reads only the three formalized fields; size, relation, and Prone state are explicit contextual inputs for this witness but cannot override the prohibition. The Rust crate accepts only the exact reviewed RLH-09 artifact identity. A temporary Rust toolchain was installed under `/tmp` and the crate's five behavioral/replay tests passed.
