use task_462::*;

#[test]
fn edge_cases() {
    assert_eq!(marks_removed_at_both_ends_colon(("", "x")), 1);
    assert_eq!(marks_removed_at_both_ends_colon((":", "z:w")), 3);
    assert_eq!(marks_removed_at_both_ends_colon(("cat", "x")), 3);
    assert_eq!(marks_removed_at_both_ends_colon(("::one:猫:ab", "z:w")), 10);
    assert_eq!(marks_removed_at_both_ends_colon(("aa:bb", "x")), 5);
    assert_eq!(marks_removed_at_both_ends_colon(("ok:🐈", "z:w")), 7);
}

#[test]
fn submission_marker() {
    assert_eq!(task_462::TRACK_PROOF, 0x52540000 | 462);
}
