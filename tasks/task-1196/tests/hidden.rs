use task_1196::*;

#[test]
fn edge_cases() {
    assert_eq!(
        remainder_after_first_mark_hash("unclosed[cat", "backup"),
        "backup"
    );
    assert_eq!(remainder_after_first_mark_hash("[]", "backup"), "backup");
    assert_eq!(remainder_after_first_mark_hash("[42]", "backup"), "backup");
    assert_eq!(remainder_after_first_mark_hash("[321]", "backup"), "backup");
    assert_eq!(
        remainder_after_first_mark_hash("hi#kit", "backup"),
        "backup"
    );
    assert_eq!(remainder_after_first_mark_hash("#é#a", "backup"), "backup");
    assert_eq!(remainder_after_first_mark_hash("", "backup"), "backup");
    assert_eq!(remainder_after_first_mark_hash("#", "backup"), "backup");
    assert_eq!(remainder_after_first_mark_hash("cat", "backup"), "backup");
    assert_eq!(
        remainder_after_first_mark_hash("##one#猫#ab", "backup"),
        "backup"
    );
    assert_eq!(remainder_after_first_mark_hash("aa#bb", "backup"), "backup");
}

#[test]
fn submission_marker() {
    assert_eq!(task_1196::TRACK_PROOF, 0x52540000 | 1196);
}
