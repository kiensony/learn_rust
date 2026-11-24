use task_310::*;

#[test]
fn edge_cases() {
    assert_eq!(closing_field_colon(String::from("")), "");
    assert_eq!(closing_field_colon(String::from(":")), "");
    assert_eq!(closing_field_colon(String::from("cat")), "cat");
    assert_eq!(closing_field_colon(String::from("::one:猫:ab")), "ab");
    assert_eq!(closing_field_colon(String::from("aa:bb")), "bb");
    assert_eq!(closing_field_colon(String::from("ok:🐈")), "🐈");
}

#[test]
fn submission_marker() {
    assert_eq!(task_310::TRACK_PROOF, 0x52540000 | 310);
}
