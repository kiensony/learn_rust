use task_991::*;

#[test]
fn edge_cases() {
    assert_eq!(
        shortest_field_with_first_tie_equals(vec!["猫=12;note", "hi=kit"]),
        vec![2usize, 2usize]
    );
    assert_eq!(
        shortest_field_with_first_tie_equals(vec![
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
            0usize, 0usize, 0usize, 3usize, 0usize, 2usize, 2usize, 2usize, 0usize, 0usize
        ]
    );
    assert_eq!(
        shortest_field_with_first_tie_equals(vec![
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
            2usize, 2usize, 2usize, 0usize, 0usize, 0usize, 3usize, 0usize, 2usize, 2usize, 2usize,
            0usize, 0usize, 2usize
        ]
    );
    assert_eq!(
        shortest_field_with_first_tie_equals(vec!["", "=", "="]),
        vec![0usize, 0usize, 0usize]
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_991::TRACK_PROOF, 0x52540000 | 991);
}
