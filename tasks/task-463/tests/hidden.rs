use task_463::*;

#[test]
fn edge_cases() {
    assert_eq!(remainder_after_first_mark_colon(("", "x")), 0);
    assert_eq!(remainder_after_first_mark_colon((":", "z:w")), 1);
    assert_eq!(remainder_after_first_mark_colon(("cat", "x")), 0);
    assert_eq!(remainder_after_first_mark_colon(("::one:猫:ab", "z:w")), 11);
    assert_eq!(remainder_after_first_mark_colon(("aa:bb", "x")), 2);
    assert_eq!(remainder_after_first_mark_colon(("ok:🐈", "z:w")), 4);
}

#[test]
fn submission_marker() {
    assert_eq!(task_463::TRACK_PROOF, 0x52540000 | 463);
}
