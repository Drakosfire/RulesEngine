use rulesengine_occupancy_witness::{
    evaluate, validate_artifact, EvaluationError, PlacementRequest, PlacementStatus, Relation, Size,
};

const ARTIFACT: &str = include_str!("../fixtures/occupancy/accepted.json");

fn request(relation: Relation, prone: bool) -> PlacementRequest {
    PlacementRequest {
        ruleset_id: "dnd5e-2024-srd-occupancy-v1".into(),
        action: "end_move_in_cell".into(),
        willing: Some(true),
        destination_occupied_by_other_creature: Some(true),
        actor_size: Some(Size::Medium),
        occupant_size: Some(Size::Medium),
        relation: Some(relation),
        occupant_prone: Some(prone),
    }
}

#[test]
fn occupied_medium_cell_rejects_for_all_relations_and_prone_states() {
    let rule = validate_artifact(ARTIFACT).unwrap();
    for relation in [Relation::Ally, Relation::Hostile, Relation::Neutral] {
        for prone in [false, true] {
            let decision = evaluate(&rule, &request(relation, prone)).unwrap();
            assert_eq!(decision.status, PlacementStatus::Reject);
            assert_eq!(decision.reason_code, Some("occupied_end_move_prohibited"));
            assert!(decision.writes.is_empty());
            assert_eq!(
                decision.applied_rule_ids,
                ["dnd5e-2024-srd-occupancy-end-move"]
            );
            assert_eq!(decision.evidence_unit_ids.len(), 1);
        }
    }
}

#[test]
fn no_restriction_from_this_rule_when_not_occupied_or_not_willing() {
    let rule = validate_artifact(ARTIFACT).unwrap();
    let mut input = request(Relation::Ally, true);
    input.destination_occupied_by_other_creature = Some(false);
    assert_eq!(
        evaluate(&rule, &input).unwrap().status,
        PlacementStatus::Accept
    );
    input.destination_occupied_by_other_creature = Some(true);
    input.willing = Some(false);
    assert_eq!(
        evaluate(&rule, &input).unwrap().status,
        PlacementStatus::Accept
    );
}

#[test]
fn missing_required_prefetch_never_guesses_a_default() {
    let rule = validate_artifact(ARTIFACT).unwrap();
    let mut input = request(Relation::Ally, true);
    input.willing = None;
    assert_eq!(
        evaluate(&rule, &input),
        Err(EvaluationError::MissingContext("willing"))
    );
    input.willing = Some(true);
    input.destination_occupied_by_other_creature = None;
    assert_eq!(
        evaluate(&rule, &input),
        Err(EvaluationError::MissingContext(
            "destination_occupied_by_other_creature"
        ))
    );
}

#[test]
fn unsupported_artifact_and_action_fail_before_decision() {
    assert_eq!(
        validate_artifact(include_str!("../fixtures/occupancy/unknown_schema.json")).err(),
        Some(EvaluationError::InvalidArtifact)
    );
    assert_eq!(
        validate_artifact(include_str!("../fixtures/occupancy/unreviewed.json")).err(),
        Some(EvaluationError::InvalidArtifact)
    );
    let rule = validate_artifact(ARTIFACT).unwrap();
    let mut input = request(Relation::Hostile, false);
    input.action = "teleport".into();
    assert_eq!(
        evaluate(&rule, &input),
        Err(EvaluationError::UnsupportedAction)
    );
}

#[test]
fn identical_input_replays_to_identical_semantic_output_and_digest() {
    let rule = validate_artifact(ARTIFACT).unwrap();
    let input = request(Relation::Ally, true);
    let first = evaluate(&rule, &input).unwrap();
    let second = evaluate(&rule, &input).unwrap();
    assert_eq!(first, second);
    assert_eq!(first.output_digest.len(), 64);
}
