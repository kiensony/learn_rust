use task_458::*;

#[test]
fn edge_cases() {
    assert_eq!(leading_field_colon(("", "x")), 1);
    assert_eq!(leading_field_colon((":", "z:w")), 1);
    assert_eq!(leading_field_colon(("cat", "x")), 3);
    assert_eq!(leading_field_colon(("::one:猫:ab", "z:w")), 0);
    assert_eq!(leading_field_colon(("aa:bb", "x")), 2);
    assert_eq!(leading_field_colon(("ok:🐈", "z:w")), 2);
}

#[test]
fn submission_marker() {
    assert_eq!(task_458::TRACK_PROOF, 0x52540000 | 458);
}
