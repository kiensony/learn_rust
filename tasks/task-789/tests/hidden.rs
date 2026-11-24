use task_789::*;

#[test]
fn edge_cases() {
    assert_eq!(
        prefix_before_final_mark_equals(&[]),
        std::collections::BTreeMap::from([])
    );
    assert_eq!(
        prefix_before_final_mark_equals(&["72;12", "72;12", "猫=12;note"]),
        std::collections::BTreeMap::from([
            (String::from(""), 2usize),
            (String::from("猫"), 1usize)
        ])
    );
    assert_eq!(
        prefix_before_final_mark_equals(&[
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
            (String::from("==one=猫"), 1usize),
            (String::from("aa"), 2usize),
            (String::from("ok"), 1usize),
            (String::from("a="), 1usize)
        ])
    );
    assert_eq!(
        prefix_before_final_mark_equals(&["", "", "=", ""]),
        std::collections::BTreeMap::from([(String::from(""), 4usize)])
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_789::TRACK_PROOF, 0x52540000 | 789);
}
