use task_1092::*;

#[test]
fn edge_cases() {
    assert_eq!(closing_field_hash(&"unclosed[cat"), 0);
    assert_eq!(closing_field_hash(&String::from("[]")), 0);
    assert_eq!(closing_field_hash(&"[42]"), 2);
    assert_eq!(closing_field_hash(&String::from("[321]")), 3);
    assert_eq!(closing_field_hash(&"hi#kit"), 0);
    assert_eq!(closing_field_hash(&String::from("#é#a")), 0);
    assert_eq!(closing_field_hash(&""), 0);
    assert_eq!(closing_field_hash(&String::from("#")), 0);
    assert_eq!(closing_field_hash(&"cat"), 0);
    assert_eq!(closing_field_hash(&String::from("##one#猫#ab")), 0);
    assert_eq!(closing_field_hash(&"aa#bb"), 0);
    assert_eq!(closing_field_hash(&String::from("ok#🐈")), 0);
}

#[test]
fn submission_marker() {
    assert_eq!(task_1092::TRACK_PROOF, 0x52540000 | 1092);
}
