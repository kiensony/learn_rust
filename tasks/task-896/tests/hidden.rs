use task_896::*;

#[test]
fn edge_cases() {
    assert_eq!(closing_marks_removed_hash("unclosed[cat"), 2);
    assert_eq!(closing_marks_removed_hash("[]"), 2);
    assert_eq!(closing_marks_removed_hash("[42]"), 4);
    assert_eq!(closing_marks_removed_hash("[321]"), 5);
    assert_eq!(closing_marks_removed_hash("hi#kit"), 2);
    assert_eq!(closing_marks_removed_hash("#é#a"), 2);
    assert_eq!(closing_marks_removed_hash(""), 2);
    assert_eq!(closing_marks_removed_hash("#"), 2);
    assert_eq!(closing_marks_removed_hash("cat"), 2);
    assert_eq!(closing_marks_removed_hash("##one#猫#ab"), 2);
    assert_eq!(closing_marks_removed_hash("aa#bb"), 2);
    assert_eq!(closing_marks_removed_hash("ok#🐈"), 2);
}

#[test]
fn submission_marker() {
    assert_eq!(task_896::TRACK_PROOF, 0x52540000 | 896);
}
