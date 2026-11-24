use task_265::*;

#[test]
fn edge_cases() {
    assert_eq!(last_field_hash(""), "");
    assert_eq!(last_field_hash("#"), "");
    assert_eq!(last_field_hash("##"), "");
    assert_eq!(last_field_hash(" café "), " café ");
    assert_eq!(last_field_hash("a#b"), "b");
    assert_eq!(last_field_hash("🐈#猫"), "猫");
}

#[test]
fn submission_marker() {
    assert_eq!(task_265::TRACK_PROOF, 0x52540000 | 265);
}
