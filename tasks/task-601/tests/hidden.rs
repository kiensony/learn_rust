use task_601::*;

#[test]
fn edge_cases() {
    assert_eq!(marks_removed_at_both_ends_hash("unclosed[cat"), "");
    assert_eq!(marks_removed_at_both_ends_hash("[]"), "");
    assert_eq!(marks_removed_at_both_ends_hash("[42]"), "42");
    assert_eq!(marks_removed_at_both_ends_hash("[321]"), "321");
    assert_eq!(marks_removed_at_both_ends_hash("hi#kit"), "");
    assert_eq!(marks_removed_at_both_ends_hash("#é#a"), "");
    assert_eq!(marks_removed_at_both_ends_hash(""), "");
    assert_eq!(marks_removed_at_both_ends_hash("#"), "");
    assert_eq!(marks_removed_at_both_ends_hash("cat"), "");
    assert_eq!(marks_removed_at_both_ends_hash("##one#猫#ab"), "");
    assert_eq!(marks_removed_at_both_ends_hash("aa#bb"), "");
    assert_eq!(marks_removed_at_both_ends_hash("ok#🐈"), "");
}

#[test]
fn submission_marker() {
    assert_eq!(task_601::TRACK_PROOF, 0x52540000 | 601);
}
