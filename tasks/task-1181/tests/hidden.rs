use task_1181::*;

#[test]
fn edge_cases() {
    assert_eq!(leading_field_equals("hi=kit", "backup"), "hi");
    assert_eq!(leading_field_equals("=é=a", "backup"), "backup");
    assert_eq!(leading_field_equals("", "backup"), "backup");
    assert_eq!(leading_field_equals("=", "backup"), "backup");
    assert_eq!(leading_field_equals("cat", "backup"), "cat");
    assert_eq!(leading_field_equals("==one=猫=ab", "backup"), "backup");
    assert_eq!(leading_field_equals("aa=bb", "backup"), "aa");
    assert_eq!(leading_field_equals("ok=🐈", "backup"), "ok");
    assert_eq!(leading_field_equals("aa=bb;meta=123", "backup"), "aa");
    assert_eq!(leading_field_equals(";ignore=all", "backup"), "backup");
    assert_eq!(leading_field_equals("a==b;", "backup"), "a");
}

#[test]
fn submission_marker() {
    assert_eq!(task_1181::TRACK_PROOF, 0x52540000 | 1181);
}
