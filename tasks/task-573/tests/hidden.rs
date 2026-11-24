use task_573::*;

#[test]
fn edge_cases() {
    assert_eq!(prefix_before_final_mark_slash("hi/kit"), "hi");
    assert_eq!(prefix_before_final_mark_slash("/é/a"), "/é");
    assert_eq!(prefix_before_final_mark_slash(""), "");
    assert_eq!(prefix_before_final_mark_slash("/"), "");
    assert_eq!(prefix_before_final_mark_slash("cat"), "");
    assert_eq!(prefix_before_final_mark_slash("//one/猫/ab"), "//one/猫");
    assert_eq!(prefix_before_final_mark_slash("aa/bb"), "aa");
    assert_eq!(prefix_before_final_mark_slash("ok/🐈"), "ok");
    assert_eq!(prefix_before_final_mark_slash("path/first/last"), "first");
    assert_eq!(prefix_before_final_mark_slash("path/path/"), "path");
    assert_eq!(prefix_before_final_mark_slash("path/"), "");
}

#[test]
fn submission_marker() {
    assert_eq!(task_573::TRACK_PROOF, 0x52540000 | 573);
}
