use task_857::*;

#[test]
fn edge_cases() {
    assert_eq!(marks_removed_at_both_ends_colon(""), 2);
    assert_eq!(marks_removed_at_both_ends_colon(":"), 2);
    assert_eq!(marks_removed_at_both_ends_colon("cat"), 5);
    assert_eq!(marks_removed_at_both_ends_colon("::one:猫:ab"), 12);
    assert_eq!(marks_removed_at_both_ends_colon("aa:bb"), 7);
    assert_eq!(marks_removed_at_both_ends_colon("ok:🐈"), 9);
}

#[test]
fn submission_marker() {
    assert_eq!(task_857::TRACK_PROOF, 0x52540000 | 857);
}
