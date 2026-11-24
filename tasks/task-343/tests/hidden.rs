use task_343::*;

#[test]
fn edge_cases() {
    assert_eq!(
        remainder_after_first_mark_equals(String::from("hi=kit")),
        "kit"
    );
    assert_eq!(
        remainder_after_first_mark_equals(String::from("=é=a")),
        "é=a"
    );
    assert_eq!(remainder_after_first_mark_equals(String::from("")), "");
    assert_eq!(remainder_after_first_mark_equals(String::from("=")), "");
    assert_eq!(remainder_after_first_mark_equals(String::from("cat")), "");
    assert_eq!(
        remainder_after_first_mark_equals(String::from("==one=猫=ab")),
        "=one=猫=ab"
    );
    assert_eq!(
        remainder_after_first_mark_equals(String::from("aa=bb")),
        "bb"
    );
    assert_eq!(
        remainder_after_first_mark_equals(String::from("ok=🐈")),
        "🐈"
    );
    assert_eq!(
        remainder_after_first_mark_equals(String::from("aa=bb;meta=123")),
        "bb"
    );
    assert_eq!(
        remainder_after_first_mark_equals(String::from(";ignore=all")),
        ""
    );
    assert_eq!(
        remainder_after_first_mark_equals(String::from("a==b;")),
        "=b"
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_343::TRACK_PROOF, 0x52540000 | 343);
}
