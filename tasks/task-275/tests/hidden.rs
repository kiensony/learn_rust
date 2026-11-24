use task_275::*;

#[test]
fn edge_cases() {
    assert_eq!(last_field_star(""), "");
    assert_eq!(last_field_star("*"), "");
    assert_eq!(last_field_star("**"), "");
    assert_eq!(last_field_star(" café "), " café ");
    assert_eq!(last_field_star("a*b"), "b");
    assert_eq!(last_field_star("🐈*猫"), "猫");
}

#[test]
fn submission_marker() {
    assert_eq!(task_275::TRACK_PROOF, 0x52540000 | 275);
}
