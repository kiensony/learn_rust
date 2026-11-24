use task_315::*;

#[test]
fn edge_cases() {
    assert_eq!(first_nonempty_field_colon(String::from("")), "");
    assert_eq!(first_nonempty_field_colon(String::from(":")), "");
    assert_eq!(first_nonempty_field_colon(String::from("cat")), "cat");
    assert_eq!(
        first_nonempty_field_colon(String::from("::one:猫:ab")),
        "one"
    );
    assert_eq!(first_nonempty_field_colon(String::from("aa:bb")), "aa");
    assert_eq!(first_nonempty_field_colon(String::from("ok:🐈")), "ok");
}

#[test]
fn submission_marker() {
    assert_eq!(task_315::TRACK_PROOF, 0x52540000 | 315);
}
