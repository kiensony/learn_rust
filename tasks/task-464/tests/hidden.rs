use task_464::*;

#[test]
fn edge_cases() {
    assert_eq!(prefix_before_final_mark_colon(("", "x")), 0);
    assert_eq!(prefix_before_final_mark_colon((":", "z:w")), 1);
    assert_eq!(prefix_before_final_mark_colon(("cat", "x")), 0);
    assert_eq!(prefix_before_final_mark_colon(("::one:猫:ab", "z:w")), 9);
    assert_eq!(prefix_before_final_mark_colon(("aa:bb", "x")), 2);
    assert_eq!(prefix_before_final_mark_colon(("ok:🐈", "z:w")), 2);
}

#[test]
fn submission_marker() {
    assert_eq!(task_464::TRACK_PROOF, 0x52540000 | 464);
}
