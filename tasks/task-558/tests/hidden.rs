use task_558::*;

#[test]
fn edge_cases() {
    assert_eq!(leading_field_colon(""), "");
    assert_eq!(leading_field_colon(":"), "");
    assert_eq!(leading_field_colon("cat"), "cat");
    assert_eq!(leading_field_colon("::one:猫:ab"), "");
    assert_eq!(leading_field_colon("aa:bb"), "aa");
    assert_eq!(leading_field_colon("ok:🐈"), "ok");
}

#[test]
fn submission_marker() {
    assert_eq!(task_558::TRACK_PROOF, 0x52540000 | 558);
}
