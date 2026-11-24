use task_1167::*;

#[test]
fn edge_cases() {
    assert_eq!(prefix_before_final_mark_slash("hi/kit", "backup"), "hi");
    assert_eq!(prefix_before_final_mark_slash("/é/a", "backup"), "/é");
    assert_eq!(prefix_before_final_mark_slash("", "backup"), "backup");
    assert_eq!(prefix_before_final_mark_slash("/", "backup"), "backup");
    assert_eq!(prefix_before_final_mark_slash("cat", "backup"), "backup");
    assert_eq!(
        prefix_before_final_mark_slash("//one/猫/ab", "backup"),
        "//one/猫"
    );
    assert_eq!(prefix_before_final_mark_slash("aa/bb", "backup"), "aa");
    assert_eq!(prefix_before_final_mark_slash("ok/🐈", "backup"), "ok");
    assert_eq!(
        prefix_before_final_mark_slash("path/first/last", "backup"),
        "first"
    );
    assert_eq!(
        prefix_before_final_mark_slash("path/path/", "backup"),
        "path"
    );
    assert_eq!(prefix_before_final_mark_slash("path/", "backup"), "backup");
}

#[test]
fn submission_marker() {
    assert_eq!(task_1167::TRACK_PROOF, 0x52540000 | 1167);
}
