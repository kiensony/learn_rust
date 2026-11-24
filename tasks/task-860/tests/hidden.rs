use task_860::*;

#[test]
fn edge_cases() {
    assert_eq!(first_nonempty_field_colon(":é:a"), 4);
    assert_eq!(first_nonempty_field_colon(":"), 2);
    assert_eq!(first_nonempty_field_colon("cat"), 5);
    assert_eq!(first_nonempty_field_colon("::one:猫:ab"), 5);
    assert_eq!(first_nonempty_field_colon("aa:bb"), 4);
    assert_eq!(first_nonempty_field_colon("ok:🐈"), 4);
}

#[test]
fn submission_marker() {
    assert_eq!(task_860::TRACK_PROOF, 0x52540000 | 860);
}
