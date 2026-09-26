//! Exact-artifact, pure occupancy legality witness for RLH-11.
//! This is not a general World Kernel or movement ruleset.

use serde::Serialize;
use serde_json::Value;
use sha2::{Digest, Sha256};

const GOLDEN: &str = include_str!("../fixtures/occupancy/accepted.json");
const RULE_ID: &str = "dnd5e-2024-srd-occupancy-end-move";
const RULESET_ID: &str = "dnd5e-2024-srd-occupancy-v1";

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EvaluationError {
    InvalidArtifact,
    WrongRuleset,
    UnsupportedAction,
    MissingContext(&'static str),
}

#[derive(Debug, Clone)]
pub struct ValidatedRule {
    artifact_digest: String,
    evidence_unit_ids: Vec<String>,
}

fn hex_sha256(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

fn parsed_and_verified(text: &str) -> Result<Value, EvaluationError> {
    let mut value: Value =
        serde_json::from_str(text).map_err(|_| EvaluationError::InvalidArtifact)?;
    let supplied = value
        .as_object_mut()
        .and_then(|object| object.remove("canonical_sha256"))
        .and_then(|digest| digest.as_str().map(str::to_owned))
        .ok_or(EvaluationError::InvalidArtifact)?;
    let canonical = serde_json::to_vec(&value).map_err(|_| EvaluationError::InvalidArtifact)?;
    if supplied != hex_sha256(&canonical) {
        return Err(EvaluationError::InvalidArtifact);
    }
    let mut golden: Value =
        serde_json::from_str(GOLDEN).map_err(|_| EvaluationError::InvalidArtifact)?;
    let expected_digest = golden
        .as_object_mut()
        .and_then(|object| object.remove("canonical_sha256"))
        .and_then(|digest| digest.as_str().map(str::to_owned))
        .ok_or(EvaluationError::InvalidArtifact)?;
    if supplied != expected_digest || value != golden {
        return Err(EvaluationError::InvalidArtifact);
    }
    Ok(value)
}

/// Load only the reviewed, immutable RLH-09 contract. No filesystem or network access.
pub fn validate_artifact(text: &str) -> Result<ValidatedRule, EvaluationError> {
    let value = parsed_and_verified(text)?;
    if value["schema_version"] != "rules_formal_artifact_v1"
        || value["rule_id"] != RULE_ID
        || value["ruleset_id"] != RULESET_ID
        || value["review"]["disposition"] != "approved"
        || value["decision"]["then"] != "reject"
        || value["decision"]["otherwise"] != "no_restriction_from_this_rule"
        || value["exceptions"] != serde_json::json!([])
    {
        return Err(EvaluationError::InvalidArtifact);
    }
    let evidence_unit_ids = value["evidence"]["evidence_unit_ids"]
        .as_array()
        .ok_or(EvaluationError::InvalidArtifact)?
        .iter()
        .map(|item| {
            item.as_str()
                .map(str::to_owned)
                .ok_or(EvaluationError::InvalidArtifact)
        })
        .collect::<Result<Vec<_>, _>>()?;
    if evidence_unit_ids.is_empty() {
        return Err(EvaluationError::InvalidArtifact);
    }
    let golden: Value =
        serde_json::from_str(GOLDEN).map_err(|_| EvaluationError::InvalidArtifact)?;
    let artifact_digest = golden["canonical_sha256"]
        .as_str()
        .ok_or(EvaluationError::InvalidArtifact)?
        .to_owned();
    Ok(ValidatedRule {
        artifact_digest,
        evidence_unit_ids,
    })
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Size {
    Tiny,
    Small,
    Medium,
    Large,
    Huge,
    Gargantuan,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Relation {
    Ally,
    Hostile,
    Neutral,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlacementRequest {
    pub ruleset_id: String,
    pub action: String,
    pub willing: Option<bool>,
    pub destination_occupied_by_other_creature: Option<bool>,
    pub actor_size: Option<Size>,
    pub occupant_size: Option<Size>,
    pub relation: Option<Relation>,
    pub occupant_prone: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PlacementStatus {
    Accept,
    Reject,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct PlacementDecision {
    pub status: PlacementStatus,
    pub reason_code: Option<&'static str>,
    pub applied_rule_ids: Vec<&'static str>,
    pub writes: Vec<String>,
    pub artifact_digest: String,
    pub evidence_unit_ids: Vec<String>,
    pub output_digest: String,
}

#[derive(Serialize)]
struct DecisionCore<'a> {
    status: &'a PlacementStatus,
    reason_code: Option<&'static str>,
    applied_rule_ids: &'a [&'static str],
    writes: &'a [String],
    artifact_digest: &'a str,
    evidence_unit_ids: &'a [String],
}

/// Pure function over an already validated artifact and explicit prefetched input.
pub fn evaluate(
    rule: &ValidatedRule,
    request: &PlacementRequest,
) -> Result<PlacementDecision, EvaluationError> {
    if request.ruleset_id != RULESET_ID {
        return Err(EvaluationError::WrongRuleset);
    }
    if request.action != "end_move_in_cell" {
        return Err(EvaluationError::UnsupportedAction);
    }
    let willing = request
        .willing
        .ok_or(EvaluationError::MissingContext("willing"))?;
    let occupied =
        request
            .destination_occupied_by_other_creature
            .ok_or(EvaluationError::MissingContext(
                "destination_occupied_by_other_creature",
            ))?;
    // Size, relation, and Prone context cannot create an exception to this sourced restriction.
    let status = if willing && occupied {
        PlacementStatus::Reject
    } else {
        PlacementStatus::Accept
    };
    let reason_code = if status == PlacementStatus::Reject {
        Some("occupied_end_move_prohibited")
    } else {
        None
    };
    let applied_rule_ids = vec![RULE_ID];
    let writes: Vec<String> = Vec::new();
    let core = DecisionCore {
        status: &status,
        reason_code,
        applied_rule_ids: &applied_rule_ids,
        writes: &writes,
        artifact_digest: &rule.artifact_digest,
        evidence_unit_ids: &rule.evidence_unit_ids,
    };
    let bytes = serde_json::to_vec(&core).map_err(|_| EvaluationError::InvalidArtifact)?;
    Ok(PlacementDecision {
        status,
        reason_code,
        applied_rule_ids,
        writes,
        artifact_digest: rule.artifact_digest.clone(),
        evidence_unit_ids: rule.evidence_unit_ids.clone(),
        output_digest: hex_sha256(&bytes),
    })
}
