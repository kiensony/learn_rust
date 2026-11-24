use task_858::*;

#[test]
fn edge_cases() {
    assert_eq!(remainder_after_first_mark_colon(""), 2);
    assert_eq!(remainder_after_first_mark_colon(":"), 2);
    assert_eq!(remainder_after_first_mark_colon("cat"), 2);
    assert_eq!(remainder_after_first_mark_colon("::one:猫:ab"), 13);
    assert_eq!(remainder_after_first_mark_colon("aa:bb"), 4);
    assert_eq!(remainder_after_first_mark_colon("ok:🐈"), 6);
}

#[test]
fn submission_marker() {
    assert_eq!(task_858::TRACK_PROOF, 0x52540000 | 858);
}
