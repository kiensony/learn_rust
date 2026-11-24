use task_565::*;

#[test]
fn edge_cases() {
    assert_eq!(longest_field_with_last_tie_colon(""), "");
    assert_eq!(longest_field_with_last_tie_colon(":"), "");
    assert_eq!(longest_field_with_last_tie_colon("cat"), "cat");
    assert_eq!(longest_field_with_last_tie_colon("::one:猫:ab"), "猫");
    assert_eq!(longest_field_with_last_tie_colon("aa:bb"), "bb");
    assert_eq!(longest_field_with_last_tie_colon("ok:🐈"), "🐈");
}

#[test]
fn submission_marker() {
    assert_eq!(task_565::TRACK_PROOF, 0x52540000 | 565);
}
