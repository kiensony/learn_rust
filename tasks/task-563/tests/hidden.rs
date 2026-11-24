use task_563::*;

#[test]
fn edge_cases() {
    assert_eq!(prefix_before_final_mark_colon(""), "");
    assert_eq!(prefix_before_final_mark_colon(":"), "");
    assert_eq!(prefix_before_final_mark_colon("cat"), "");
    assert_eq!(prefix_before_final_mark_colon("::one:猫:ab"), "::one:猫");
    assert_eq!(prefix_before_final_mark_colon("aa:bb"), "aa");
    assert_eq!(prefix_before_final_mark_colon("ok:🐈"), "ok");
}

#[test]
fn submission_marker() {
    assert_eq!(task_563::TRACK_PROOF, 0x52540000 | 563);
}
