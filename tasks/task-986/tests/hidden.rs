use task_986::*;

#[test]
fn edge_cases() {
    assert_eq!(marks_removed_at_both_ends_equals(vec![]), vec![]);
    assert_eq!(
        marks_removed_at_both_ends_equals(vec![
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
            4usize, 0usize, 0usize, 3usize, 10usize, 5usize, 7usize, 5usize, 0usize, 4usize
        ]
    );
    assert_eq!(
        marks_removed_at_both_ends_equals(vec![
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
            2usize, 6usize, 6usize, 4usize, 0usize, 0usize, 3usize, 10usize, 5usize, 7usize,
            5usize, 0usize, 4usize, 2usize
        ]
    );
    assert_eq!(
        marks_removed_at_both_ends_equals(vec!["", "=", "="]),
        vec![0usize, 0usize, 0usize]
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_986::TRACK_PROOF, 0x52540000 | 986);
}
