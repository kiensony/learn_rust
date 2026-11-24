use task_544::*;

#[test]
fn edge_cases() {
    assert_eq!(prefix_before_final_mark_equals("hi=kit"), 2);
    assert_eq!(prefix_before_final_mark_equals("=é=a"), 2);
    assert_eq!(prefix_before_final_mark_equals(""), 0);
    assert_eq!(prefix_before_final_mark_equals("="), 0);
    assert_eq!(prefix_before_final_mark_equals("cat"), 0);
    assert_eq!(prefix_before_final_mark_equals("==one=猫=ab"), 7);
    assert_eq!(prefix_before_final_mark_equals("aa=bb"), 2);
    assert_eq!(prefix_before_final_mark_equals("ok=🐈"), 2);
    assert_eq!(prefix_before_final_mark_equals("aa=bb;meta=123"), 2);
    assert_eq!(prefix_before_final_mark_equals(";ignore=all"), 0);
    assert_eq!(prefix_before_final_mark_equals("a==b;"), 2);
}

#[test]
fn submission_marker() {
    assert_eq!(task_544::TRACK_PROOF, 0x52540000 | 544);
}
