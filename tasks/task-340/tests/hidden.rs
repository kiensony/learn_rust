use task_340::*;

#[test]
fn edge_cases() {
    assert_eq!(
        leading_marks_removed_equals(String::from("hi=kit")),
        "hi=kit"
    );
    assert_eq!(leading_marks_removed_equals(String::from("=é=a")), "é=a");
    assert_eq!(leading_marks_removed_equals(String::from("")), "");
    assert_eq!(leading_marks_removed_equals(String::from("=")), "");
    assert_eq!(leading_marks_removed_equals(String::from("cat")), "cat");
    assert_eq!(
        leading_marks_removed_equals(String::from("==one=猫=ab")),
        "one=猫=ab"
    );
    assert_eq!(leading_marks_removed_equals(String::from("aa=bb")), "aa=bb");
    assert_eq!(leading_marks_removed_equals(String::from("ok=🐈")), "ok=🐈");
    assert_eq!(
        leading_marks_removed_equals(String::from("aa=bb;meta=123")),
        "aa=bb"
    );
    assert_eq!(
        leading_marks_removed_equals(String::from(";ignore=all")),
        ""
    );
    assert_eq!(leading_marks_removed_equals(String::from("a==b;")), "a==b");
}

#[test]
fn submission_marker() {
    assert_eq!(task_340::TRACK_PROOF, 0x52540000 | 340);
}
