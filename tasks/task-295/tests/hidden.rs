use task_295::*;

#[test]
fn edge_cases() {
    assert_eq!(last_field_plus(""), "");
    assert_eq!(last_field_plus("+"), "");
    assert_eq!(last_field_plus("++"), "");
    assert_eq!(last_field_plus(" café "), " café ");
    assert_eq!(last_field_plus("a+b"), "b");
    assert_eq!(last_field_plus("🐈+猫"), "猫");
}

#[test]
fn submission_marker() {
    assert_eq!(task_295::TRACK_PROOF, 0x52540000 | 295);
}
