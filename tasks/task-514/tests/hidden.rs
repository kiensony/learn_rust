use task_514::*;

#[test]
fn edge_cases() {
    assert_eq!(prefix_before_final_mark_colon(":é:a"), 2);
    assert_eq!(prefix_before_final_mark_colon(":"), 0);
    assert_eq!(prefix_before_final_mark_colon("cat"), 0);
    assert_eq!(prefix_before_final_mark_colon("::one:猫:ab"), 7);
    assert_eq!(prefix_before_final_mark_colon("aa:bb"), 2);
    assert_eq!(prefix_before_final_mark_colon("ok:🐈"), 2);
}

#[test]
fn submission_marker() {
    assert_eq!(task_514::TRACK_PROOF, 0x52540000 | 514);
}
