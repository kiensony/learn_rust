use task_309::*;

#[test]
fn edge_cases() {
    assert_eq!(leading_field_colon(String::from("")), "");
    assert_eq!(leading_field_colon(String::from(":")), "");
    assert_eq!(leading_field_colon(String::from("cat")), "cat");
    assert_eq!(leading_field_colon(String::from("::one:猫:ab")), "");
    assert_eq!(leading_field_colon(String::from("aa:bb")), "aa");
    assert_eq!(leading_field_colon(String::from("ok:🐈")), "ok");
}

#[test]
fn submission_marker() {
    assert_eq!(task_309::TRACK_PROOF, 0x52540000 | 309);
}
