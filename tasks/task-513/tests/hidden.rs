use task_513::*;

#[test]
fn edge_cases() {
    assert_eq!(remainder_after_first_mark_colon(":é:a"), 3);
    assert_eq!(remainder_after_first_mark_colon(":"), 0);
    assert_eq!(remainder_after_first_mark_colon("cat"), 0);
    assert_eq!(remainder_after_first_mark_colon("::one:猫:ab"), 9);
    assert_eq!(remainder_after_first_mark_colon("aa:bb"), 2);
    assert_eq!(remainder_after_first_mark_colon("ok:🐈"), 1);
}

#[test]
fn submission_marker() {
    assert_eq!(task_513::TRACK_PROOF, 0x52540000 | 513);
}
