use task_784::*;

#[test]
fn edge_cases() {
    assert_eq!(
        closing_field_equals(&[]),
        std::collections::BTreeMap::from([])
    );
    assert_eq!(
        closing_field_equals(&["72;12", "72;12", "猫=12;note"]),
        std::collections::BTreeMap::from([
            (String::from("72"), 2usize),
            (String::from("12"), 1usize)
        ])
    );
    assert_eq!(
        closing_field_equals(&[
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
            (String::from("ab"), 1usize),
            (String::from("bb"), 2usize),
            (String::from("🐈"), 1usize),
            (String::from("b"), 1usize)
        ])
    );
    assert_eq!(
        closing_field_equals(&["", "", "=", ""]),
        std::collections::BTreeMap::from([(String::from(""), 4usize)])
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_784::TRACK_PROOF, 0x52540000 | 784);
}
