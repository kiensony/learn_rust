use task_854::*;

#[test]
fn edge_cases() {
    assert_eq!(closing_field_colon(""), 2);
    assert_eq!(closing_field_colon(":"), 2);
    assert_eq!(closing_field_colon("cat"), 5);
    assert_eq!(closing_field_colon("::one:猫:ab"), 4);
    assert_eq!(closing_field_colon("aa:bb"), 4);
    assert_eq!(closing_field_colon("ok:🐈"), 6);
}

#[test]
fn submission_marker() {
    assert_eq!(task_854::TRACK_PROOF, 0x52540000 | 854);
}
