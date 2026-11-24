use task_705::*;

#[test]
fn edge_cases() {
    assert_eq!(leading_field_colon(Some(":é:a")), None);
    assert_eq!(leading_field_colon(Some("")), None);
    assert_eq!(leading_field_colon(Some(":")), None);
    assert_eq!(leading_field_colon(Some("cat")), None);
    assert_eq!(leading_field_colon(Some("::one:猫:ab")), None);
    assert_eq!(leading_field_colon(Some("aa:bb")), None);
    assert_eq!(leading_field_colon(Some("ok:🐈")), None);
    assert_eq!(leading_field_colon(None), None);
    assert_eq!(leading_field_colon(Some(":9")), None);
    assert_eq!(leading_field_colon(Some("3300")), Some(3300));
    assert_eq!(leading_field_colon(Some("65536")), None);
}

#[test]
fn submission_marker() {
    assert_eq!(task_705::TRACK_PROOF, 0x52540000 | 705);
}
