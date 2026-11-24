use task_474::*;

#[test]
fn edge_cases() {
    assert_eq!(prefix_before_final_mark_slash(("hi/kit", "x")), 2);
    assert_eq!(prefix_before_final_mark_slash(("/é/a", "z/w")), 3);
    assert_eq!(prefix_before_final_mark_slash(("", "x")), 0);
    assert_eq!(prefix_before_final_mark_slash(("/", "z/w")), 1);
    assert_eq!(prefix_before_final_mark_slash(("cat", "x")), 0);
    assert_eq!(prefix_before_final_mark_slash(("//one/猫/ab", "z/w")), 9);
    assert_eq!(prefix_before_final_mark_slash(("aa/bb", "x")), 2);
    assert_eq!(prefix_before_final_mark_slash(("ok/🐈", "z/w")), 2);
    assert_eq!(prefix_before_final_mark_slash(("path/first/last", "x")), 5);
    assert_eq!(prefix_before_final_mark_slash(("path/path/", "z/w")), 4);
    assert_eq!(prefix_before_final_mark_slash(("path/", "x")), 0);
}

#[test]
fn submission_marker() {
    assert_eq!(task_474::TRACK_PROOF, 0x52540000 | 474);
}
