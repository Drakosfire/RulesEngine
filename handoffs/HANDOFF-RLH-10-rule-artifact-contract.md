# HANDOFF — RLH-10 rule artifact contract

**Status:** DEFERRED DRAFT  
**Repository:** `Drakosfire/RulesEngine`  
**Authority:** `Drakosfire/DungeonOverMind/Docs/Plans/PLAN-rules-lawyer-graph-experiment.md`  
**Predecessor:** `RLH_09_REVIEWED_RULE_ARTIFACT_ACCEPTED`  
**Primary question:** What is the smallest exact artifact contract RulesEngine can accept for the reviewed occupancy rule while preserving deterministic/pure runtime constraints?  
**Unlocks:** RLH-11

## Re-anchor

RulesEngine is specification-first today. Re-read the authoritative rule-authoring constraints, evaluation pipeline, world-kernel contract/invariants, and non-goals before adding anything.

This PR is contract/fixture work. Do not bootstrap a large runtime.

## Contract

Import the exact RLH-09 accepted occupancy artifact as a golden external fixture and define the RulesEngine-side acceptance schema/validation needed to consume it.

The accepted representation must be:

- versioned;
- deterministic;
- free of natural-language interpretation requirements at runtime;
- bounded in required reads;
- explicit about input fields;
- explicit about conditions and result/effect;
- provenance-bearing for audit while evidence remains non-executable metadata.

Unknown schema versions, unknown operations, missing required reads, unsupported condition forms, and unreviewed artifacts fail closed.

## Mapping to existing invariants

The artifact must satisfy:

- pure rule evaluation;
- bounded prefetch;
- no graph traversal;
- no IO/model calls;
- no hidden authority;
- deterministic identical-input output.

If RLH-09 violates these, reject and hand back the mismatch. Do not weaken RulesEngine to make the demo pass.

## Suggested lease

Keep this small:

```text
handoffs/HANDOFF-RLH-10-rule-artifact-contract.md
contracts/ or schemas/
fixtures/occupancy/
minimal validation tests/harness if justified
```

## Required fixtures

- accepted reviewed occupancy artifact;
- same artifact without approval;
- unknown schema version;
- unsupported condition/op;
- missing required provenance reference;
- free-form executable payload attempt.

## Acceptance

Produce one field-by-field mapping to the existing Rule Authoring Constraints, marking each field executable or audit-only.

Acceptance token:

```text
RLH_10_RULE_ARTIFACT_CONTRACT_ACCEPTED
```

## Stop condition

If safe consumption requires weakening foundational RulesEngine invariants, stop for architecture review.
