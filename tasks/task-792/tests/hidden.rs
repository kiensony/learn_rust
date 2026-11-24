use task_792::*;

#[test]
fn edge_cases() {
    assert_eq!(
        shortest_field_with_first_tie_equals(&[]),
        std::collections::BTreeMap::from([])
    );
    assert_eq!(
        shortest_field_with_first_tie_equals(&["72;12", "72;12", "猫=12;note"]),
        std::collections::BTreeMap::from([
            (String::from("72"), 2usize),
            (String::from("12"), 1usize)
        ])
    );
    assert_eq!(
        shortest_field_with_first_tie_equals(&[
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
            (String::from(""), 5usize),
            (String::from("cat"), 1usize),
            (String::from("aa"), 2usize),
            (String::from("ok"), 1usize)
        ])
    );
    assert_eq!(
        shortest_field_with_first_tie_equals(&["", "", "=", ""]),
        std::collections::BTreeMap::from([(String::from(""), 4usize)])
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_792::TRACK_PROOF, 0x52540000 | 792);
}
