use task_898::*;

#[test]
fn edge_cases() {
    assert_eq!(remainder_after_first_mark_hash("unclosed[cat"), 2);
    assert_eq!(remainder_after_first_mark_hash("[]"), 2);
    assert_eq!(remainder_after_first_mark_hash("[42]"), 2);
    assert_eq!(remainder_after_first_mark_hash("[321]"), 2);
    assert_eq!(remainder_after_first_mark_hash("hi#kit"), 2);
    assert_eq!(remainder_after_first_mark_hash("#é#a"), 2);
    assert_eq!(remainder_after_first_mark_hash(""), 2);
    assert_eq!(remainder_after_first_mark_hash("#"), 2);
    assert_eq!(remainder_after_first_mark_hash("cat"), 2);
    assert_eq!(remainder_after_first_mark_hash("##one#猫#ab"), 2);
    assert_eq!(remainder_after_first_mark_hash("aa#bb"), 2);
}

#[test]
fn submission_marker() {
    assert_eq!(task_898::TRACK_PROOF, 0x52540000 | 898);
}
