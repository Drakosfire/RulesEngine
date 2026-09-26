import copy
from hashlib import sha256
import json
from pathlib import Path
import unittest

from contracts.validate_rule_artifact import ArtifactRejected, canonical_bytes, load_validated, validate_artifact


FIXTURE = Path(__file__).resolve().parents[1] / "fixtures/occupancy"


class ArtifactContractTests(unittest.TestCase):
    def test_golden_artifact_is_accepted(self):
        value = load_validated(FIXTURE / "accepted.json")
        self.assertEqual(value["decision"]["then"], "reject")

    def test_required_bad_fixtures_fail_closed(self):
        for name in ("unreviewed", "unknown_schema", "unsupported_operation",
                     "missing_provenance", "freeform_payload"):
            with self.subTest(name=name), self.assertRaises(ArtifactRejected):
                load_validated(FIXTURE / f"{name}.json")

    def test_semantic_gate_cannot_be_bypassed_by_rehashing(self):
        artifact = copy.deepcopy(load_validated(FIXTURE / "accepted.json"))
        artifact["inputs"]["dynamic_graph_query"] = {"type": "string"}
        artifact["canonical_sha256"] = sha256(canonical_bytes({
            key: value for key, value in artifact.items() if key != "canonical_sha256"
        })).hexdigest()
        with self.assertRaises(ArtifactRejected):
            validate_artifact(artifact)


if __name__ == "__main__":
    unittest.main()
