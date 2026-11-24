use task_499::*;

#[test]
fn edge_cases() {
    assert_eq!(closing_field_hash(("unclosed[cat", "x")), 0);
    assert_eq!(closing_field_hash(("[]", "z#w")), 0);
    assert_eq!(closing_field_hash(("[42]", "x")), 2);
    assert_eq!(closing_field_hash(("[321]", "z#w")), 3);
    assert_eq!(closing_field_hash(("hi#kit", "x")), 0);
    assert_eq!(closing_field_hash(("#é#a", "z#w")), 0);
    assert_eq!(closing_field_hash(("", "x")), 0);
    assert_eq!(closing_field_hash(("#", "z#w")), 0);
    assert_eq!(closing_field_hash(("cat", "x")), 0);
    assert_eq!(closing_field_hash(("##one#猫#ab", "z#w")), 0);
    assert_eq!(closing_field_hash(("aa#bb", "x")), 0);
    assert_eq!(closing_field_hash(("ok#🐈", "z#w")), 0);
}

#[test]
fn submission_marker() {
    assert_eq!(task_499::TRACK_PROOF, 0x52540000 | 499);
}
