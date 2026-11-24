use task_548::*;

#[test]
fn edge_cases() {
    assert_eq!(leading_field_hash("unclosed[cat"), 0);
    assert_eq!(leading_field_hash("[]"), 0);
    assert_eq!(leading_field_hash("[42]"), 2);
    assert_eq!(leading_field_hash("[321]"), 3);
    assert_eq!(leading_field_hash("hi#kit"), 0);
    assert_eq!(leading_field_hash("#é#a"), 0);
    assert_eq!(leading_field_hash(""), 0);
    assert_eq!(leading_field_hash("#"), 0);
    assert_eq!(leading_field_hash("cat"), 0);
    assert_eq!(leading_field_hash("##one#猫#ab"), 0);
    assert_eq!(leading_field_hash("aa#bb"), 0);
    assert_eq!(leading_field_hash("ok#🐈"), 0);
}

#[test]
fn submission_marker() {
    assert_eq!(task_548::TRACK_PROOF, 0x52540000 | 548);
}
