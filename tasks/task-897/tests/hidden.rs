use task_897::*;

#[test]
fn edge_cases() {
    assert_eq!(marks_removed_at_both_ends_hash("unclosed[cat"), 2);
    assert_eq!(marks_removed_at_both_ends_hash("[]"), 2);
    assert_eq!(marks_removed_at_both_ends_hash("[42]"), 4);
    assert_eq!(marks_removed_at_both_ends_hash("[321]"), 5);
    assert_eq!(marks_removed_at_both_ends_hash("hi#kit"), 2);
    assert_eq!(marks_removed_at_both_ends_hash("#é#a"), 2);
    assert_eq!(marks_removed_at_both_ends_hash(""), 2);
    assert_eq!(marks_removed_at_both_ends_hash("#"), 2);
    assert_eq!(marks_removed_at_both_ends_hash("cat"), 2);
    assert_eq!(marks_removed_at_both_ends_hash("##one#猫#ab"), 2);
    assert_eq!(marks_removed_at_both_ends_hash("aa#bb"), 2);
    assert_eq!(marks_removed_at_both_ends_hash("ok#🐈"), 2);
}

#[test]
fn submission_marker() {
    assert_eq!(task_897::TRACK_PROOF, 0x52540000 | 897);
}
