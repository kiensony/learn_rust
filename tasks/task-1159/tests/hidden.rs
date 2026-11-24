use task_1159::*;

#[test]
fn edge_cases() {
    assert_eq!(longest_field_with_last_tie_colon("", "backup"), "backup");
    assert_eq!(longest_field_with_last_tie_colon(":", "backup"), "backup");
    assert_eq!(longest_field_with_last_tie_colon("cat", "backup"), "cat");
    assert_eq!(
        longest_field_with_last_tie_colon("::one:猫:ab", "backup"),
        "猫"
    );
    assert_eq!(longest_field_with_last_tie_colon("aa:bb", "backup"), "bb");
    assert_eq!(longest_field_with_last_tie_colon("ok:🐈", "backup"), "🐈");
}

#[test]
fn submission_marker() {
    assert_eq!(task_1159::TRACK_PROOF, 0x52540000 | 1159);
}
