use task_502::*;

#[test]
fn edge_cases() {
    assert_eq!(marks_removed_at_both_ends_hash(("unclosed[cat", "x")), 0);
    assert_eq!(marks_removed_at_both_ends_hash(("[]", "z#w")), 0);
    assert_eq!(marks_removed_at_both_ends_hash(("[42]", "x")), 2);
    assert_eq!(marks_removed_at_both_ends_hash(("[321]", "z#w")), 3);
    assert_eq!(marks_removed_at_both_ends_hash(("hi#kit", "x")), 0);
    assert_eq!(marks_removed_at_both_ends_hash(("#é#a", "z#w")), 0);
    assert_eq!(marks_removed_at_both_ends_hash(("", "x")), 0);
    assert_eq!(marks_removed_at_both_ends_hash(("#", "z#w")), 0);
    assert_eq!(marks_removed_at_both_ends_hash(("cat", "x")), 0);
    assert_eq!(marks_removed_at_both_ends_hash(("##one#猫#ab", "z#w")), 0);
    assert_eq!(marks_removed_at_both_ends_hash(("aa#bb", "x")), 0);
    assert_eq!(marks_removed_at_both_ends_hash(("ok#🐈", "z#w")), 0);
}

#[test]
fn submission_marker() {
    assert_eq!(task_502::TRACK_PROOF, 0x52540000 | 502);
}
