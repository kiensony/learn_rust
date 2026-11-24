use task_957::*;

#[test]
fn edge_cases() {
    assert_eq!(marks_removed_at_both_ends_colon(vec![]), vec![]);
    assert_eq!(
        marks_removed_at_both_ends_colon(vec![":", "cat", "::one:猫:ab", "aa:bb", "ok:🐈"]),
        vec![0usize, 3usize, 10usize, 5usize, 7usize]
    );
    assert_eq!(
        marks_removed_at_both_ends_colon(vec![
            "hi:kit",
            ":é:a",
            "",
            ":",
            "cat",
            "::one:猫:ab",
            "aa:bb",
            "ok:🐈",
            "hi:kit"
        ]),
        vec![
            6usize, 4usize, 0usize, 0usize, 3usize, 10usize, 5usize, 7usize, 6usize
        ]
    );
    assert_eq!(
        marks_removed_at_both_ends_colon(vec!["", ":", ":"]),
        vec![0usize, 0usize, 0usize]
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_957::TRACK_PROOF, 0x52540000 | 957);
}
