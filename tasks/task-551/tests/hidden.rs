use task_551::*;

#[test]
fn edge_cases() {
    assert_eq!(closing_marks_removed_hash("unclosed[cat"), 0);
    assert_eq!(closing_marks_removed_hash("[]"), 0);
    assert_eq!(closing_marks_removed_hash("[42]"), 2);
    assert_eq!(closing_marks_removed_hash("[321]"), 3);
    assert_eq!(closing_marks_removed_hash("hi#kit"), 0);
    assert_eq!(closing_marks_removed_hash("#é#a"), 0);
    assert_eq!(closing_marks_removed_hash(""), 0);
    assert_eq!(closing_marks_removed_hash("#"), 0);
    assert_eq!(closing_marks_removed_hash("cat"), 0);
    assert_eq!(closing_marks_removed_hash("##one#猫#ab"), 0);
    assert_eq!(closing_marks_removed_hash("aa#bb"), 0);
    assert_eq!(closing_marks_removed_hash("ok#🐈"), 0);
}

#[test]
fn submission_marker() {
    assert_eq!(task_551::TRACK_PROOF, 0x52540000 | 551);
}
