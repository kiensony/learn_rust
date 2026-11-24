use task_564::*;

#[test]
fn edge_cases() {
    assert_eq!(first_nonempty_field_colon(""), "");
    assert_eq!(first_nonempty_field_colon(":"), "");
    assert_eq!(first_nonempty_field_colon("cat"), "cat");
    assert_eq!(first_nonempty_field_colon("::one:猫:ab"), "one");
    assert_eq!(first_nonempty_field_colon("aa:bb"), "aa");
    assert_eq!(first_nonempty_field_colon("ok:🐈"), "ok");
}

#[test]
fn submission_marker() {
    assert_eq!(task_564::TRACK_PROOF, 0x52540000 | 564);
}
