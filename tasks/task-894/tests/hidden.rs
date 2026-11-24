use task_894::*;

#[test]
fn edge_cases() {
    assert_eq!(closing_field_hash("unclosed[cat"), 2);
    assert_eq!(closing_field_hash("[]"), 2);
    assert_eq!(closing_field_hash("[42]"), 4);
    assert_eq!(closing_field_hash("[321]"), 5);
    assert_eq!(closing_field_hash("hi#kit"), 2);
    assert_eq!(closing_field_hash("#é#a"), 2);
    assert_eq!(closing_field_hash(""), 2);
    assert_eq!(closing_field_hash("#"), 2);
    assert_eq!(closing_field_hash("cat"), 2);
    assert_eq!(closing_field_hash("##one#猫#ab"), 2);
    assert_eq!(closing_field_hash("aa#bb"), 2);
    assert_eq!(closing_field_hash("ok#🐈"), 2);
}

#[test]
fn submission_marker() {
    assert_eq!(task_894::TRACK_PROOF, 0x52540000 | 894);
}
