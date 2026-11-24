use task_1182::*;

#[test]
fn edge_cases() {
    assert_eq!(closing_field_equals("hi=kit", "backup"), "kit");
    assert_eq!(closing_field_equals("=é=a", "backup"), "a");
    assert_eq!(closing_field_equals("", "backup"), "backup");
    assert_eq!(closing_field_equals("=", "backup"), "backup");
    assert_eq!(closing_field_equals("cat", "backup"), "cat");
    assert_eq!(closing_field_equals("==one=猫=ab", "backup"), "ab");
    assert_eq!(closing_field_equals("aa=bb", "backup"), "bb");
    assert_eq!(closing_field_equals("ok=🐈", "backup"), "🐈");
    assert_eq!(closing_field_equals("aa=bb;meta=123", "backup"), "bb");
    assert_eq!(closing_field_equals(";ignore=all", "backup"), "backup");
    assert_eq!(closing_field_equals("a==b;", "backup"), "b");
}

#[test]
fn submission_marker() {
    assert_eq!(task_1182::TRACK_PROOF, 0x52540000 | 1182);
}
