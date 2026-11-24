use task_597::*;

#[test]
fn edge_cases() {
    assert_eq!(leading_field_hash("unclosed[cat"), "");
    assert_eq!(leading_field_hash("[]"), "");
    assert_eq!(leading_field_hash("[42]"), "42");
    assert_eq!(leading_field_hash("[321]"), "321");
    assert_eq!(leading_field_hash("hi#kit"), "");
    assert_eq!(leading_field_hash("#é#a"), "");
    assert_eq!(leading_field_hash(""), "");
    assert_eq!(leading_field_hash("#"), "");
    assert_eq!(leading_field_hash("cat"), "");
    assert_eq!(leading_field_hash("##one#猫#ab"), "");
    assert_eq!(leading_field_hash("aa#bb"), "");
    assert_eq!(leading_field_hash("ok#🐈"), "");
}

#[test]
fn submission_marker() {
    assert_eq!(task_597::TRACK_PROOF, 0x52540000 | 597);
}
