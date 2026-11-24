use task_277::*;

#[test]
fn edge_cases() {
    assert_eq!(count_separators_star(""), "0");
    assert_eq!(count_separators_star("*"), "1");
    assert_eq!(count_separators_star("**"), "2");
    assert_eq!(count_separators_star(" café "), "0");
    assert_eq!(count_separators_star("a*b"), "1");
    assert_eq!(count_separators_star("🐈*猫"), "1");
}

#[test]
fn submission_marker() {
    assert_eq!(task_277::TRACK_PROOF, 0x52540000 | 277);
}
