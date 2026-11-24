use task_740::*;

#[test]
fn edge_cases() {
    assert_eq!(prefix_before_final_mark_equals(Some("猫=12;note")), None);
    assert_eq!(prefix_before_final_mark_equals(Some("hi=kit")), None);
    assert_eq!(prefix_before_final_mark_equals(Some("=é=a")), None);
    assert_eq!(prefix_before_final_mark_equals(Some("")), None);
    assert_eq!(prefix_before_final_mark_equals(Some("=")), None);
    assert_eq!(prefix_before_final_mark_equals(Some("cat")), None);
    assert_eq!(prefix_before_final_mark_equals(Some("==one=猫=ab")), None);
    assert_eq!(prefix_before_final_mark_equals(Some("aa=bb")), None);
    assert_eq!(prefix_before_final_mark_equals(Some("ok=🐈")), None);
    assert_eq!(
        prefix_before_final_mark_equals(Some("aa=bb;meta=123")),
        None
    );
    assert_eq!(prefix_before_final_mark_equals(Some(";ignore=all")), None);
    assert_eq!(prefix_before_final_mark_equals(Some("a==b;")), None);
    assert_eq!(prefix_before_final_mark_equals(None), None);
    assert_eq!(prefix_before_final_mark_equals(Some("=9")), None);
    assert_eq!(prefix_before_final_mark_equals(Some("3300")), None);
    assert_eq!(prefix_before_final_mark_equals(Some("65536")), None);
}

#[test]
fn submission_marker() {
    assert_eq!(task_740::TRACK_PROOF, 0x52540000 | 740);
}
