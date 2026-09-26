# RLH-10 occupancy artifact field mapping

The imported golden fixture is the exact RLH-09 artifact at RulesIngestion `cc1d56b`.
Acceptance uses `contracts/validate_rule_artifact.py`; the runtime must repeat these checks before evaluating an untrusted artifact.

| Artifact field | Role | Rule Authoring Constraints mapping |
| --- | --- | --- |
| `schema_version`, `rule_id`, `rule_version`, `ruleset_id`, `ruleset_version` | gate | Version stability and active ruleset scope |
| `inputs.action`, `inputs.willing` | executable | Explicit TurnInput parameters |
| `inputs.destination_occupied_by_other_creature` | executable | Bounded prefetched field; no dynamic lookup |
| `decision.when.all[].input/equals` | executable | Fixed pure Boolean predicates over those reads |
| `decision.then`, `decision.otherwise` | executable | Reject or emit zero writes; no direct mutation |
| `exceptions` | executable gate | Must be empty for the corrected source rule |
| `statement` | audit only | Reviewed human explanation; never parsed at runtime |
| `evidence.*` | audit only | Exact source/knowledge identity; never queried in evaluation |
| `compiler.*` | gate and audit | Fixed accepted candidate; graph closure must be false |
| `review.*` | gate and audit | Approval required, scoped to this rule |
| `canonical_sha256` | gate | Stable artifact bytes and replay identity |

The legality check produces no writes and never advances a world frame. Missing prefetch fields are errors. The accepted decision does not authorize other movement rules or a general placement engine.
