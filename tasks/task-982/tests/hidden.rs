use task_982::*;

#[test]
fn edge_cases() {
    assert_eq!(leading_field_equals(vec![]), vec![]);
    assert_eq!(
        leading_field_equals(vec![
            "=é=a",
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
        vec![
            0usize, 0usize, 0usize, 3usize, 0usize, 2usize, 2usize, 2usize, 0usize, 1usize
        ]
    );
    assert_eq!(
        leading_field_equals(vec![
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
        vec![
            2usize, 3usize, 2usize, 0usize, 0usize, 0usize, 3usize, 0usize, 2usize, 2usize, 2usize,
            0usize, 1usize, 2usize
        ]
    );
    assert_eq!(
        leading_field_equals(vec!["", "=", "="]),
        vec![0usize, 0usize, 0usize]
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_982::TRACK_PROOF, 0x52540000 | 982);
}
