use task_512::*;

#[test]
fn edge_cases() {
    assert_eq!(marks_removed_at_both_ends_colon(""), 0);
    assert_eq!(marks_removed_at_both_ends_colon(":"), 0);
    assert_eq!(marks_removed_at_both_ends_colon("cat"), 3);
    assert_eq!(marks_removed_at_both_ends_colon("::one:猫:ab"), 8);
    assert_eq!(marks_removed_at_both_ends_colon("aa:bb"), 5);
    assert_eq!(marks_removed_at_both_ends_colon("ok:🐈"), 4);
}

#[test]
fn submission_marker() {
    assert_eq!(task_512::TRACK_PROOF, 0x52540000 | 512);
}
