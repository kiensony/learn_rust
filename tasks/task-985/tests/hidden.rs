use task_985::*;

#[test]
fn edge_cases() {
    assert_eq!(closing_marks_removed_equals(vec![]), vec![]);
    assert_eq!(
        closing_marks_removed_equals(vec![
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
            5usize, 0usize, 0usize, 3usize, 12usize, 5usize, 7usize, 5usize, 0usize, 4usize
        ]
    );
    assert_eq!(
        closing_marks_removed_equals(vec![
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
            2usize, 6usize, 6usize, 5usize, 0usize, 0usize, 3usize, 12usize, 5usize, 7usize,
            5usize, 0usize, 4usize, 2usize
        ]
    );
    assert_eq!(
        closing_marks_removed_equals(vec!["", "=", "="]),
        vec![0usize, 0usize, 0usize]
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_985::TRACK_PROOF, 0x52540000 | 985);
}
