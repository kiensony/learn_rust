use task_593::*;

#[test]
fn edge_cases() {
    assert_eq!(prefix_before_final_mark_equals("hi=kit"), "hi");
    assert_eq!(prefix_before_final_mark_equals("=é=a"), "=é");
    assert_eq!(prefix_before_final_mark_equals(""), "");
    assert_eq!(prefix_before_final_mark_equals("="), "");
    assert_eq!(prefix_before_final_mark_equals("cat"), "");
    assert_eq!(prefix_before_final_mark_equals("==one=猫=ab"), "==one=猫");
    assert_eq!(prefix_before_final_mark_equals("aa=bb"), "aa");
    assert_eq!(prefix_before_final_mark_equals("ok=🐈"), "ok");
    assert_eq!(prefix_before_final_mark_equals("aa=bb;meta=123"), "aa");
    assert_eq!(prefix_before_final_mark_equals(";ignore=all"), "");
    assert_eq!(prefix_before_final_mark_equals("a==b;"), "a=");
}

#[test]
fn submission_marker() {
    assert_eq!(task_593::TRACK_PROOF, 0x52540000 | 593);
}
