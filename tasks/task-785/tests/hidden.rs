use task_785::*;

#[test]
fn edge_cases() {
    assert_eq!(
        leading_marks_removed_equals(&[]),
        std::collections::BTreeMap::from([])
    );
    assert_eq!(
        leading_marks_removed_equals(&["72;12", "72;12", "猫=12;note"]),
        std::collections::BTreeMap::from([
            (String::from("72"), 2usize),
            (String::from("猫=12"), 1usize)
        ])
    );
    assert_eq!(
        leading_marks_removed_equals(&[
            "",
            "=",
            "cat",
            "==one=猫=ab",
            "aa=bb",
            "ok=🐈",
            "aa=bb;meta=123",
            ";ignore=all",
            "a==b;"
        ]),
        std::collections::BTreeMap::from([
            (String::from(""), 3usize),
            (String::from("cat"), 1usize),
            (String::from("one=猫=ab"), 1usize),
            (String::from("aa=bb"), 2usize),
            (String::from("ok=🐈"), 1usize),
            (String::from("a==b"), 1usize)
        ])
    );
    assert_eq!(
        leading_marks_removed_equals(&["", "", "=", ""]),
        std::collections::BTreeMap::from([(String::from(""), 4usize)])
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_785::TRACK_PROOF, 0x52540000 | 785);
}
