use task_344::*;

#[test]
fn edge_cases() {
    assert_eq!(
        prefix_before_final_mark_equals(String::from("hi=kit")),
        "hi"
    );
    assert_eq!(prefix_before_final_mark_equals(String::from("=é=a")), "=é");
    assert_eq!(prefix_before_final_mark_equals(String::from("")), "");
    assert_eq!(prefix_before_final_mark_equals(String::from("=")), "");
    assert_eq!(prefix_before_final_mark_equals(String::from("cat")), "");
    assert_eq!(
        prefix_before_final_mark_equals(String::from("==one=猫=ab")),
        "==one=猫"
    );
    assert_eq!(prefix_before_final_mark_equals(String::from("aa=bb")), "aa");
    assert_eq!(prefix_before_final_mark_equals(String::from("ok=🐈")), "ok");
    assert_eq!(
        prefix_before_final_mark_equals(String::from("aa=bb;meta=123")),
        "aa"
    );
    assert_eq!(
        prefix_before_final_mark_equals(String::from(";ignore=all")),
        ""
    );
    assert_eq!(prefix_before_final_mark_equals(String::from("a==b;")), "a=");
}

#[test]
fn submission_marker() {
    assert_eq!(task_344::TRACK_PROOF, 0x52540000 | 344);
}
