use task_600::*;

#[test]
fn edge_cases() {
    assert_eq!(closing_marks_removed_hash("unclosed[cat"), "");
    assert_eq!(closing_marks_removed_hash("[]"), "");
    assert_eq!(closing_marks_removed_hash("[42]"), "42");
    assert_eq!(closing_marks_removed_hash("[321]"), "321");
    assert_eq!(closing_marks_removed_hash("hi#kit"), "");
    assert_eq!(closing_marks_removed_hash("#é#a"), "");
    assert_eq!(closing_marks_removed_hash(""), "");
    assert_eq!(closing_marks_removed_hash("#"), "");
    assert_eq!(closing_marks_removed_hash("cat"), "");
    assert_eq!(closing_marks_removed_hash("##one#猫#ab"), "");
    assert_eq!(closing_marks_removed_hash("aa#bb"), "");
    assert_eq!(closing_marks_removed_hash("ok#🐈"), "");
}

#[test]
fn submission_marker() {
    assert_eq!(task_600::TRACK_PROOF, 0x52540000 | 600);
}
