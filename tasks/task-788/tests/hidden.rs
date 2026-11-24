use task_788::*;

#[test]
fn edge_cases() {
    assert_eq!(
        remainder_after_first_mark_equals(&[]),
        std::collections::BTreeMap::from([])
    );
    assert_eq!(
        remainder_after_first_mark_equals(&["72;12", "72;12", "猫=12;note"]),
        std::collections::BTreeMap::from([
            (String::from(""), 2usize),
            (String::from("12"), 1usize)
        ])
    );
    assert_eq!(
        remainder_after_first_mark_equals(&[
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
            (String::from(""), 4usize),
            (String::from("=one=猫=ab"), 1usize),
            (String::from("bb"), 2usize),
            (String::from("🐈"), 1usize),
            (String::from("=b"), 1usize)
        ])
    );
    assert_eq!(
        remainder_after_first_mark_equals(&["", "", "=", ""]),
        std::collections::BTreeMap::from([(String::from(""), 4usize)])
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_788::TRACK_PROOF, 0x52540000 | 788);
}
