use task_297::*;

#[test]
fn edge_cases() {
    assert_eq!(count_separators_plus(""), "0");
    assert_eq!(count_separators_plus("+"), "1");
    assert_eq!(count_separators_plus("++"), "2");
    assert_eq!(count_separators_plus(" café "), "0");
    assert_eq!(count_separators_plus("a+b"), "1");
    assert_eq!(count_separators_plus("🐈+猫"), "1");
}

#[test]
fn submission_marker() {
    assert_eq!(task_297::TRACK_PROOF, 0x52540000 | 297);
}
