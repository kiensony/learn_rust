use task_1033::*;

#[test]
fn edge_cases() {
    assert_eq!(closing_field_equals(["hi=kit"]), [3]);
    assert_eq!(
        closing_field_equals([
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
        [1, 0, 0, 3, 2, 2, 4, 2, 0, 1]
    );
    assert_eq!(
        closing_field_equals([
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
        [2, 2, 3, 1, 0, 0, 3, 2, 2, 4, 2, 0, 1, 2]
    );
    assert_eq!(
        closing_field_equals([String::from(""), String::from("="), String::from("=")]),
        [0, 0, 0]
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_1033::TRACK_PROOF, 0x52540000 | 1033);
}
