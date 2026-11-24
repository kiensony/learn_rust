use task_1087::*;

#[test]
fn edge_cases() {
    assert_eq!(prefix_before_final_mark_equals(&"hi=kit"), 2);
    assert_eq!(prefix_before_final_mark_equals(&String::from("=é=a")), 3);
    assert_eq!(prefix_before_final_mark_equals(&""), 0);
    assert_eq!(prefix_before_final_mark_equals(&String::from("=")), 0);
    assert_eq!(prefix_before_final_mark_equals(&"cat"), 0);
    assert_eq!(
        prefix_before_final_mark_equals(&String::from("==one=猫=ab")),
        9
    );
    assert_eq!(prefix_before_final_mark_equals(&"aa=bb"), 2);
    assert_eq!(prefix_before_final_mark_equals(&String::from("ok=🐈")), 2);
    assert_eq!(prefix_before_final_mark_equals(&"aa=bb;meta=123"), 2);
    assert_eq!(
        prefix_before_final_mark_equals(&String::from(";ignore=all")),
        0
    );
    assert_eq!(prefix_before_final_mark_equals(&"a==b;"), 2);
}

#[test]
fn submission_marker() {
    assert_eq!(task_1087::TRACK_PROOF, 0x52540000 | 1087);
}
