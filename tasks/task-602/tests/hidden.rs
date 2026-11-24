use task_602::*;

#[test]
fn edge_cases() {
    assert_eq!(remainder_after_first_mark_hash("unclosed[cat"), "");
    assert_eq!(remainder_after_first_mark_hash("[]"), "");
    assert_eq!(remainder_after_first_mark_hash("[42]"), "");
    assert_eq!(remainder_after_first_mark_hash("[321]"), "");
    assert_eq!(remainder_after_first_mark_hash("hi#kit"), "");
    assert_eq!(remainder_after_first_mark_hash("#é#a"), "");
    assert_eq!(remainder_after_first_mark_hash(""), "");
    assert_eq!(remainder_after_first_mark_hash("#"), "");
    assert_eq!(remainder_after_first_mark_hash("cat"), "");
    assert_eq!(remainder_after_first_mark_hash("##one#猫#ab"), "");
    assert_eq!(remainder_after_first_mark_hash("aa#bb"), "");
}

#[test]
fn submission_marker() {
    assert_eq!(task_602::TRACK_PROOF, 0x52540000 | 602);
}
