use task_503::*;

#[test]
fn edge_cases() {
    assert_eq!(remainder_after_first_mark_hash(("unclosed[cat", "x")), 0);
    assert_eq!(remainder_after_first_mark_hash(("[]", "z#w")), 0);
    assert_eq!(remainder_after_first_mark_hash(("[42]", "x")), 0);
    assert_eq!(remainder_after_first_mark_hash(("[321]", "z#w")), 0);
    assert_eq!(remainder_after_first_mark_hash(("hi#kit", "x")), 0);
    assert_eq!(remainder_after_first_mark_hash(("#é#a", "z#w")), 0);
    assert_eq!(remainder_after_first_mark_hash(("", "x")), 0);
    assert_eq!(remainder_after_first_mark_hash(("#", "z#w")), 0);
    assert_eq!(remainder_after_first_mark_hash(("cat", "x")), 0);
    assert_eq!(remainder_after_first_mark_hash(("##one#猫#ab", "z#w")), 0);
    assert_eq!(remainder_after_first_mark_hash(("aa#bb", "x")), 0);
}

#[test]
fn submission_marker() {
    assert_eq!(task_503::TRACK_PROOF, 0x52540000 | 503);
}
