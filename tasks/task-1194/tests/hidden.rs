use task_1194::*;

#[test]
fn edge_cases() {
    assert_eq!(
        closing_marks_removed_hash("unclosed[cat", "backup"),
        "backup"
    );
    assert_eq!(closing_marks_removed_hash("[]", "backup"), "backup");
    assert_eq!(closing_marks_removed_hash("[42]", "backup"), "42");
    assert_eq!(closing_marks_removed_hash("[321]", "backup"), "321");
    assert_eq!(closing_marks_removed_hash("hi#kit", "backup"), "backup");
    assert_eq!(closing_marks_removed_hash("#é#a", "backup"), "backup");
    assert_eq!(closing_marks_removed_hash("", "backup"), "backup");
    assert_eq!(closing_marks_removed_hash("#", "backup"), "backup");
    assert_eq!(closing_marks_removed_hash("cat", "backup"), "backup");
    assert_eq!(
        closing_marks_removed_hash("##one#猫#ab", "backup"),
        "backup"
    );
    assert_eq!(closing_marks_removed_hash("aa#bb", "backup"), "backup");
    assert_eq!(closing_marks_removed_hash("ok#🐈", "backup"), "backup");
}

#[test]
fn submission_marker() {
    assert_eq!(task_1194::TRACK_PROOF, 0x52540000 | 1194);
}
