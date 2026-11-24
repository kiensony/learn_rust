use task_975::*;

#[test]
fn edge_cases() {
    assert_eq!(closing_marks_removed_pipe(vec![]), vec![]);
    assert_eq!(
        closing_marks_removed_pipe(vec![
            "|é|a",
            "",
            "|",
            "cat",
            "||one|猫|ab",
            "aa|bb",
            "ok|🐈",
            "header\nleft|right\r\n",
            "header\n\n",
            "猫|x\nend|a"
        ]),
        vec![
            5usize, 0usize, 0usize, 3usize, 12usize, 5usize, 7usize, 10usize, 0usize, 5usize
        ]
    );
    assert_eq!(
        closing_marks_removed_pipe(vec![
            "first\n123",
            "header\n猫|123",
            "hi|kit",
            "|é|a",
            "",
            "|",
            "cat",
            "||one|猫|ab",
            "aa|bb",
            "ok|🐈",
            "header\nleft|right\r\n",
            "header\n\n",
            "猫|x\nend|a",
            "first\n123"
        ]),
        vec![
            3usize, 7usize, 6usize, 5usize, 0usize, 0usize, 3usize, 12usize, 5usize, 7usize,
            10usize, 0usize, 5usize, 3usize
        ]
    );
    assert_eq!(
        closing_marks_removed_pipe(vec!["", "|", "|"]),
        vec![0usize, 0usize, 0usize]
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_975::TRACK_PROOF, 0x52540000 | 975);
}
