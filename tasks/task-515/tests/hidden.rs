use task_515::*;

#[test]
fn edge_cases() {
    assert_eq!(first_nonempty_field_colon(""), 0);
    assert_eq!(first_nonempty_field_colon(":"), 0);
    assert_eq!(first_nonempty_field_colon("cat"), 3);
    assert_eq!(first_nonempty_field_colon("::one:猫:ab"), 3);
    assert_eq!(first_nonempty_field_colon("aa:bb"), 2);
    assert_eq!(first_nonempty_field_colon("ok:🐈"), 2);
}

#[test]
fn submission_marker() {
    assert_eq!(task_515::TRACK_PROOF, 0x52540000 | 515);
}
