use task_1191::*;

#[test]
fn edge_cases() {
    assert_eq!(leading_field_hash("unclosed[cat", "backup"), "backup");
    assert_eq!(leading_field_hash("[]", "backup"), "backup");
    assert_eq!(leading_field_hash("[42]", "backup"), "42");
    assert_eq!(leading_field_hash("[321]", "backup"), "321");
    assert_eq!(leading_field_hash("hi#kit", "backup"), "backup");
    assert_eq!(leading_field_hash("#é#a", "backup"), "backup");
    assert_eq!(leading_field_hash("", "backup"), "backup");
    assert_eq!(leading_field_hash("#", "backup"), "backup");
    assert_eq!(leading_field_hash("cat", "backup"), "backup");
    assert_eq!(leading_field_hash("##one#猫#ab", "backup"), "backup");
    assert_eq!(leading_field_hash("aa#bb", "backup"), "backup");
    assert_eq!(leading_field_hash("ok#🐈", "backup"), "backup");
}

#[test]
fn submission_marker() {
    assert_eq!(task_1191::TRACK_PROOF, 0x52540000 | 1191);
}
