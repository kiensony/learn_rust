use task_1036::*;

#[test]
fn edge_cases() {
    assert_eq!(marks_removed_at_both_ends_equals(["hi=kit"]), [6]);
    assert_eq!(
        marks_removed_at_both_ends_equals([
            String::from("=é=a"),
            String::from(""),
            String::from("="),
            String::from("cat"),
            String::from("==one=猫=ab"),
            String::from("aa=bb"),
            String::from("ok=🐈"),
            String::from("aa=bb;meta=123"),
            String::from(";ignore=all"),
            String::from("a==b;")
        ]),
        [4, 0, 0, 3, 10, 5, 7, 5, 0, 4]
    );
    assert_eq!(
        marks_removed_at_both_ends_equals([
            "72;12",
            "猫=12;note",
            "hi=kit",
            "=é=a",
            "",
            "=",
            "cat",
            "==one=猫=ab",
            "aa=bb",
            "ok=🐈",
            "aa=bb;meta=123",
            ";ignore=all",
            "a==b;",
            "72;12"
        ]),
        [2, 6, 6, 4, 0, 0, 3, 10, 5, 7, 5, 0, 4, 2]
    );
    assert_eq!(
        marks_removed_at_both_ends_equals([String::from(""), String::from("="), String::from("=")]),
        [0, 0, 0]
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_1036::TRACK_PROOF, 0x52540000 | 1036);
}
