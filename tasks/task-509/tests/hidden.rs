use task_509::*;

#[test]
fn edge_cases() {
    assert_eq!(closing_field_colon(""), 0);
    assert_eq!(closing_field_colon(":"), 0);
    assert_eq!(closing_field_colon("cat"), 3);
    assert_eq!(closing_field_colon("::one:猫:ab"), 2);
    assert_eq!(closing_field_colon("aa:bb"), 2);
    assert_eq!(closing_field_colon("ok:🐈"), 1);
}

#[test]
fn submission_marker() {
    assert_eq!(task_509::TRACK_PROOF, 0x52540000 | 509);
}
