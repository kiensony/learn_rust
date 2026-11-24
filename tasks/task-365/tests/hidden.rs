use task_365::*;

#[test]
fn edge_cases() {
    assert_eq!(first_nonempty_field_colon(&String::from(":é:a")), 2);
    assert_eq!(first_nonempty_field_colon(&String::from(":")), 0);
    assert_eq!(first_nonempty_field_colon(&String::from("cat")), 3);
    assert_eq!(first_nonempty_field_colon(&String::from("::one:猫:ab")), 3);
    assert_eq!(first_nonempty_field_colon(&String::from("aa:bb")), 2);
    assert_eq!(first_nonempty_field_colon(&String::from("ok:🐈")), 2);
}

#[test]
fn submission_marker() {
    assert_eq!(task_365::TRACK_PROOF, 0x52540000 | 365);
}
