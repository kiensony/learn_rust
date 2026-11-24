use task_393::*;

#[test]
fn edge_cases() {
    assert_eq!(
        remainder_after_first_mark_equals(&String::from("hi=kit")),
        3
    );
    assert_eq!(remainder_after_first_mark_equals(&String::from("=é=a")), 4);
    assert_eq!(remainder_after_first_mark_equals(&String::from("")), 0);
    assert_eq!(remainder_after_first_mark_equals(&String::from("=")), 0);
    assert_eq!(remainder_after_first_mark_equals(&String::from("cat")), 0);
    assert_eq!(
        remainder_after_first_mark_equals(&String::from("==one=猫=ab")),
        11
    );
    assert_eq!(remainder_after_first_mark_equals(&String::from("aa=bb")), 2);
    assert_eq!(remainder_after_first_mark_equals(&String::from("ok=🐈")), 4);
    assert_eq!(
        remainder_after_first_mark_equals(&String::from("aa=bb;meta=123")),
        2
    );
    assert_eq!(
        remainder_after_first_mark_equals(&String::from(";ignore=all")),
        0
    );
    assert_eq!(remainder_after_first_mark_equals(&String::from("a==b;")), 2);
}

#[test]
fn submission_marker() {
    assert_eq!(task_393::TRACK_PROOF, 0x52540000 | 393);
}
