use task_1189::*;

#[test]
fn edge_cases() {
    assert_eq!(
        longest_field_with_last_tie_equals("hi=kit", "backup"),
        "kit"
    );
    assert_eq!(longest_field_with_last_tie_equals("=é=a", "backup"), "é");
    assert_eq!(longest_field_with_last_tie_equals("", "backup"), "backup");
    assert_eq!(longest_field_with_last_tie_equals("=", "backup"), "backup");
    assert_eq!(longest_field_with_last_tie_equals("cat", "backup"), "cat");
    assert_eq!(
        longest_field_with_last_tie_equals("==one=猫=ab", "backup"),
        "猫"
    );
    assert_eq!(longest_field_with_last_tie_equals("aa=bb", "backup"), "bb");
    assert_eq!(longest_field_with_last_tie_equals("ok=🐈", "backup"), "🐈");
    assert_eq!(
        longest_field_with_last_tie_equals("aa=bb;meta=123", "backup"),
        "bb"
    );
    assert_eq!(
        longest_field_with_last_tie_equals(";ignore=all", "backup"),
        "backup"
    );
    assert_eq!(longest_field_with_last_tie_equals("a==b;", "backup"), "b");
}

#[test]
fn submission_marker() {
    assert_eq!(task_1189::TRACK_PROOF, 0x52540000 | 1189);
}
