use task_1187::*;

#[test]
fn edge_cases() {
    assert_eq!(prefix_before_final_mark_equals("hi=kit", "backup"), "hi");
    assert_eq!(prefix_before_final_mark_equals("=é=a", "backup"), "=é");
    assert_eq!(prefix_before_final_mark_equals("", "backup"), "backup");
    assert_eq!(prefix_before_final_mark_equals("=", "backup"), "backup");
    assert_eq!(prefix_before_final_mark_equals("cat", "backup"), "backup");
    assert_eq!(
        prefix_before_final_mark_equals("==one=猫=ab", "backup"),
        "==one=猫"
    );
    assert_eq!(prefix_before_final_mark_equals("aa=bb", "backup"), "aa");
    assert_eq!(prefix_before_final_mark_equals("ok=🐈", "backup"), "ok");
    assert_eq!(
        prefix_before_final_mark_equals("aa=bb;meta=123", "backup"),
        "aa"
    );
    assert_eq!(
        prefix_before_final_mark_equals(";ignore=all", "backup"),
        "backup"
    );
    assert_eq!(prefix_before_final_mark_equals("a==b;", "backup"), "a=");
}

#[test]
fn submission_marker() {
    assert_eq!(task_1187::TRACK_PROOF, 0x52540000 | 1187);
}
