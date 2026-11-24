use task_524::*;

#[test]
fn edge_cases() {
    assert_eq!(prefix_before_final_mark_slash("hi/kit"), 2);
    assert_eq!(prefix_before_final_mark_slash("/é/a"), 2);
    assert_eq!(prefix_before_final_mark_slash(""), 0);
    assert_eq!(prefix_before_final_mark_slash("/"), 0);
    assert_eq!(prefix_before_final_mark_slash("cat"), 0);
    assert_eq!(prefix_before_final_mark_slash("//one/猫/ab"), 7);
    assert_eq!(prefix_before_final_mark_slash("aa/bb"), 2);
    assert_eq!(prefix_before_final_mark_slash("ok/🐈"), 2);
    assert_eq!(prefix_before_final_mark_slash("path/first/last"), 5);
    assert_eq!(prefix_before_final_mark_slash("path/path/"), 4);
    assert_eq!(prefix_before_final_mark_slash("path/"), 0);
}

#[test]
fn submission_marker() {
    assert_eq!(task_524::TRACK_PROOF, 0x52540000 | 524);
}
