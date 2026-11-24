use task_559::*;

#[test]
fn edge_cases() {
    assert_eq!(closing_field_colon(""), "");
    assert_eq!(closing_field_colon(":"), "");
    assert_eq!(closing_field_colon("cat"), "cat");
    assert_eq!(closing_field_colon("::one:猫:ab"), "ab");
    assert_eq!(closing_field_colon("aa:bb"), "bb");
    assert_eq!(closing_field_colon("ok:🐈"), "🐈");
}

#[test]
fn submission_marker() {
    assert_eq!(task_559::TRACK_PROOF, 0x52540000 | 559);
}
