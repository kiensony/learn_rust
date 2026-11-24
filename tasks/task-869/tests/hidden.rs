use task_869::*;

#[test]
fn edge_cases() {
    assert_eq!(prefix_before_final_mark_slash("hi/kit"), 4);
    assert_eq!(prefix_before_final_mark_slash("/é/a"), 5);
    assert_eq!(prefix_before_final_mark_slash(""), 2);
    assert_eq!(prefix_before_final_mark_slash("/"), 2);
    assert_eq!(prefix_before_final_mark_slash("cat"), 2);
    assert_eq!(prefix_before_final_mark_slash("//one/猫/ab"), 11);
    assert_eq!(prefix_before_final_mark_slash("aa/bb"), 4);
    assert_eq!(prefix_before_final_mark_slash("ok/🐈"), 4);
    assert_eq!(prefix_before_final_mark_slash("path/first/last"), 7);
    assert_eq!(prefix_before_final_mark_slash("path/path/"), 6);
    assert_eq!(prefix_before_final_mark_slash("path/"), 2);
}

#[test]
fn submission_marker() {
    assert_eq!(task_869::TRACK_PROOF, 0x52540000 | 869);
}
