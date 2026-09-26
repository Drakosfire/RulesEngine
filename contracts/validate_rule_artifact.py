"""Offline acceptance gate for the exact RLH-09 occupancy artifact subset."""

from __future__ import annotations

from hashlib import sha256
import json
from pathlib import Path


EXPECTED_INPUTS = {
    "action": {"type": "enum", "values": ["end_move_in_cell"]},
    "willing": {"type": "boolean"},
    "destination_occupied_by_other_creature": {"type": "boolean"},
}
EXPECTED_CONDITIONS = [
    {"input": "action", "equals": "end_move_in_cell"},
    {"input": "willing", "equals": True},
    {"input": "destination_occupied_by_other_creature", "equals": True},
]
REQUIRED_KEYS = {
    "schema_version", "rule_id", "rule_version", "ruleset_id", "ruleset_version",
    "statement", "inputs", "decision", "exceptions", "evidence", "compiler", "review",
    "canonical_sha256",
}
REQUIRED_EVIDENCE = {
    "evidence_unit_ids", "source_uri", "source_artifact_id", "source_revision_id",
    "dungeonmind_space_id", "dungeonmind_revision_id", "dungeonmind_assertion_ids",
    "evidence_ref_ids",
}


class ArtifactRejected(ValueError):
    pass


def canonical_bytes(value: dict) -> bytes:
    return json.dumps(value, sort_keys=True, ensure_ascii=False,
                      separators=(",", ":")).encode("utf-8")


def validate_artifact(value: object) -> dict:
    if not isinstance(value, dict) or set(value) != REQUIRED_KEYS:
        raise ArtifactRejected("unknown or missing top-level field")
    if value["schema_version"] != "rules_formal_artifact_v1":
        raise ArtifactRejected("unknown schema version")
    if (value["rule_id"] != "dnd5e-2024-srd-occupancy-end-move"
            or value["rule_version"] != "1"
            or value["ruleset_id"] != "dnd5e-2024-srd-occupancy-v1"
            or value["ruleset_version"] != "srd-5.2.1"):
        raise ArtifactRejected("unsupported rule identity")
    if value["inputs"] != EXPECTED_INPUTS:
        raise ArtifactRejected("unsupported or missing required reads")
    if value["decision"] != {
        "when": {"all": EXPECTED_CONDITIONS},
        "then": "reject", "otherwise": "no_restriction_from_this_rule",
    } or value["exceptions"] != []:
        raise ArtifactRejected("unsupported condition, operation, or exception")
    evidence = value["evidence"]
    if not isinstance(evidence, dict) or set(evidence) != REQUIRED_EVIDENCE:
        raise ArtifactRejected("missing required provenance reference")
    if any(not isinstance(evidence[name], list) or not evidence[name]
           or any(not isinstance(item, str) or not item for item in evidence[name])
           for name in ("evidence_unit_ids", "dungeonmind_assertion_ids", "evidence_ref_ids")):
        raise ArtifactRejected("missing exact evidence identity")
    if any(not isinstance(evidence[name], str) or not evidence[name]
           for name in ("source_uri", "source_artifact_id", "source_revision_id",
                        "dungeonmind_space_id", "dungeonmind_revision_id")):
        raise ArtifactRejected("missing source identity")
    if not isinstance(value["statement"], str) or not value["statement"]:
        raise ArtifactRejected("missing reviewed statement")
    compiler = value["compiler"]
    if (not isinstance(compiler, dict) or set(compiler) != {
            "id", "input_candidate_ids", "graph_closure_used"}
            or compiler["graph_closure_used"] is not False
            or compiler["input_candidate_ids"] != ["occ-f3debc71f994db571810"]):
        raise ArtifactRejected("unsupported compiler or hidden graph authority")
    review = value["review"]
    if (not isinstance(review, dict) or review.get("disposition") != "approved"
            or review.get("approved_rule_id") != value["rule_id"]
            or review.get("source") != "srd-5.2.1-human-gold-and-user-correction"):
        raise ArtifactRejected("unreviewed artifact")
    digest = value["canonical_sha256"]
    if not isinstance(digest, str) or sha256(canonical_bytes({
            key: item for key, item in value.items() if key != "canonical_sha256"
    })).hexdigest() != digest:
        raise ArtifactRejected("canonical digest mismatch")
    return value


def load_validated(path: Path) -> dict:
    return validate_artifact(json.loads(path.read_text(encoding="utf-8")))
