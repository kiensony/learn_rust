use task_987::*;

#[test]
fn edge_cases() {
    assert_eq!(remainder_after_first_mark_equals(vec![]), vec![]);
    assert_eq!(
        remainder_after_first_mark_equals(vec![
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
            4usize, 0usize, 0usize, 0usize, 11usize, 2usize, 4usize, 2usize, 0usize, 2usize
        ]
    );
    assert_eq!(
        remainder_after_first_mark_equals(vec![
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
            0usize, 2usize, 3usize, 4usize, 0usize, 0usize, 0usize, 11usize, 2usize, 4usize,
            2usize, 0usize, 2usize, 0usize
        ]
    );
    assert_eq!(
        remainder_after_first_mark_equals(vec!["", "=", "="]),
        vec![0usize, 0usize, 0usize]
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_987::TRACK_PROOF, 0x52540000 | 987);
}
