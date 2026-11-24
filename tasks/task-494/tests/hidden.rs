use task_494::*;

#[test]
fn edge_cases() {
    assert_eq!(prefix_before_final_mark_equals(("hi=kit", "x")), 2);
    assert_eq!(prefix_before_final_mark_equals(("=é=a", "z=w")), 3);
    assert_eq!(prefix_before_final_mark_equals(("", "x")), 0);
    assert_eq!(prefix_before_final_mark_equals(("=", "z=w")), 1);
    assert_eq!(prefix_before_final_mark_equals(("cat", "x")), 0);
    assert_eq!(prefix_before_final_mark_equals(("==one=猫=ab", "z=w")), 9);
    assert_eq!(prefix_before_final_mark_equals(("aa=bb", "x")), 2);
    assert_eq!(prefix_before_final_mark_equals(("ok=🐈", "z=w")), 2);
    assert_eq!(prefix_before_final_mark_equals(("aa=bb;meta=123", "x")), 2);
    assert_eq!(prefix_before_final_mark_equals((";ignore=all", "z=w")), 0);
    assert_eq!(prefix_before_final_mark_equals(("a==b;", "x")), 2);
}

#[test]
fn submission_marker() {
    assert_eq!(task_494::TRACK_PROOF, 0x52540000 | 494);
}
