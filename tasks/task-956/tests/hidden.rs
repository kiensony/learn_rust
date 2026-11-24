use task_956::*;

#[test]
fn edge_cases() {
    assert_eq!(closing_marks_removed_colon(vec![]), vec![]);
    assert_eq!(
        closing_marks_removed_colon(vec![":", "cat", "::one:猫:ab", "aa:bb", "ok:🐈"]),
        vec![0usize, 3usize, 12usize, 5usize, 7usize]
    );
    assert_eq!(
        closing_marks_removed_colon(vec![
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
            6usize, 5usize, 0usize, 0usize, 3usize, 12usize, 5usize, 7usize, 6usize
        ]
    );
    assert_eq!(
        closing_marks_removed_colon(vec!["", ":", ":"]),
        vec![0usize, 0usize, 0usize]
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_956::TRACK_PROOF, 0x52540000 | 956);
}
