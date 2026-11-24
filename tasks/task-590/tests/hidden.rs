use task_590::*;

#[test]
fn edge_cases() {
    assert_eq!(closing_marks_removed_equals("hi=kit"), "hi=kit");
    assert_eq!(closing_marks_removed_equals("=é=a"), "=é=a");
    assert_eq!(closing_marks_removed_equals(""), "");
    assert_eq!(closing_marks_removed_equals("="), "");
    assert_eq!(closing_marks_removed_equals("cat"), "cat");
    assert_eq!(closing_marks_removed_equals("==one=猫=ab"), "==one=猫=ab");
    assert_eq!(closing_marks_removed_equals("aa=bb"), "aa=bb");
    assert_eq!(closing_marks_removed_equals("ok=🐈"), "ok=🐈");
    assert_eq!(closing_marks_removed_equals("aa=bb;meta=123"), "aa=bb");
    assert_eq!(closing_marks_removed_equals(";ignore=all"), "");
    assert_eq!(closing_marks_removed_equals("a==b;"), "a==b");
}

#[test]
fn submission_marker() {
    assert_eq!(task_590::TRACK_PROOF, 0x52540000 | 590);
}
