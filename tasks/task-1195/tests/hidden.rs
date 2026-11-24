use task_1195::*;

#[test]
fn edge_cases() {
    assert_eq!(
        marks_removed_at_both_ends_hash("unclosed[cat", "backup"),
        "backup"
    );
    assert_eq!(marks_removed_at_both_ends_hash("[]", "backup"), "backup");
    assert_eq!(marks_removed_at_both_ends_hash("[42]", "backup"), "42");
    assert_eq!(marks_removed_at_both_ends_hash("[321]", "backup"), "321");
    assert_eq!(
        marks_removed_at_both_ends_hash("hi#kit", "backup"),
        "backup"
    );
    assert_eq!(marks_removed_at_both_ends_hash("#é#a", "backup"), "backup");
    assert_eq!(marks_removed_at_both_ends_hash("", "backup"), "backup");
    assert_eq!(marks_removed_at_both_ends_hash("#", "backup"), "backup");
    assert_eq!(marks_removed_at_both_ends_hash("cat", "backup"), "backup");
    assert_eq!(
        marks_removed_at_both_ends_hash("##one#猫#ab", "backup"),
        "backup"
    );
    assert_eq!(marks_removed_at_both_ends_hash("aa#bb", "backup"), "backup");
    assert_eq!(marks_removed_at_both_ends_hash("ok#🐈", "backup"), "backup");
}

#[test]
fn submission_marker() {
    assert_eq!(task_1195::TRACK_PROOF, 0x52540000 | 1195);
}
