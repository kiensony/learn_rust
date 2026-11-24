use task_1053::*;

#[test]
fn edge_cases() {
    assert_eq!(closing_field_colon(&""), 0);
    assert_eq!(closing_field_colon(&String::from(":")), 0);
    assert_eq!(closing_field_colon(&"cat"), 3);
    assert_eq!(closing_field_colon(&String::from("::one:猫:ab")), 2);
    assert_eq!(closing_field_colon(&"aa:bb"), 2);
    assert_eq!(closing_field_colon(&String::from("ok:🐈")), 4);
}

#[test]
fn submission_marker() {
    assert_eq!(task_1053::TRACK_PROOF, 0x52540000 | 1053);
}
