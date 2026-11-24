use task_553::*;

#[test]
fn edge_cases() {
    assert_eq!(remainder_after_first_mark_hash("unclosed[cat"), 0);
    assert_eq!(remainder_after_first_mark_hash("[]"), 0);
    assert_eq!(remainder_after_first_mark_hash("[42]"), 0);
    assert_eq!(remainder_after_first_mark_hash("[321]"), 0);
    assert_eq!(remainder_after_first_mark_hash("hi#kit"), 0);
    assert_eq!(remainder_after_first_mark_hash("#é#a"), 0);
    assert_eq!(remainder_after_first_mark_hash(""), 0);
    assert_eq!(remainder_after_first_mark_hash("#"), 0);
    assert_eq!(remainder_after_first_mark_hash("cat"), 0);
    assert_eq!(remainder_after_first_mark_hash("##one#猫#ab"), 0);
    assert_eq!(remainder_after_first_mark_hash("aa#bb"), 0);
}

#[test]
fn submission_marker() {
    assert_eq!(task_553::TRACK_PROOF, 0x52540000 | 553);
}
