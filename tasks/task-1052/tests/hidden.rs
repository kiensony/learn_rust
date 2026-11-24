use task_1052::*;

#[test]
fn edge_cases() {
    assert_eq!(leading_field_colon(&""), 0);
    assert_eq!(leading_field_colon(&String::from(":")), 0);
    assert_eq!(leading_field_colon(&"cat"), 3);
    assert_eq!(leading_field_colon(&String::from("::one:猫:ab")), 0);
    assert_eq!(leading_field_colon(&"aa:bb"), 2);
    assert_eq!(leading_field_colon(&String::from("ok:🐈")), 2);
}

#[test]
fn submission_marker() {
    assert_eq!(task_1052::TRACK_PROOF, 0x52540000 | 1052);
}
