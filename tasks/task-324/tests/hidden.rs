use task_324::*;

#[test]
fn edge_cases() {
    assert_eq!(prefix_before_final_mark_slash(String::from("hi/kit")), "hi");
    assert_eq!(prefix_before_final_mark_slash(String::from("/é/a")), "/é");
    assert_eq!(prefix_before_final_mark_slash(String::from("")), "");
    assert_eq!(prefix_before_final_mark_slash(String::from("/")), "");
    assert_eq!(prefix_before_final_mark_slash(String::from("cat")), "");
    assert_eq!(
        prefix_before_final_mark_slash(String::from("//one/猫/ab")),
        "//one/猫"
    );
    assert_eq!(prefix_before_final_mark_slash(String::from("aa/bb")), "aa");
    assert_eq!(prefix_before_final_mark_slash(String::from("ok/🐈")), "ok");
    assert_eq!(
        prefix_before_final_mark_slash(String::from("path/first/last")),
        "first"
    );
    assert_eq!(
        prefix_before_final_mark_slash(String::from("path/path/")),
        "path"
    );
    assert_eq!(prefix_before_final_mark_slash(String::from("path/")), "");
}

#[test]
fn submission_marker() {
    assert_eq!(task_324::TRACK_PROOF, 0x52540000 | 324);
}
