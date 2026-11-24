use task_1157::*;

#[test]
fn edge_cases() {
    assert_eq!(prefix_before_final_mark_colon("", "backup"), "backup");
    assert_eq!(prefix_before_final_mark_colon(":", "backup"), "backup");
    assert_eq!(prefix_before_final_mark_colon("cat", "backup"), "backup");
    assert_eq!(
        prefix_before_final_mark_colon("::one:猫:ab", "backup"),
        "::one:猫"
    );
    assert_eq!(prefix_before_final_mark_colon("aa:bb", "backup"), "aa");
    assert_eq!(prefix_before_final_mark_colon("ok:🐈", "backup"), "ok");
}

#[test]
fn submission_marker() {
    assert_eq!(task_1157::TRACK_PROOF, 0x52540000 | 1157);
}
