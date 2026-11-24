use task_974::*;

#[test]
fn edge_cases() {
    assert_eq!(leading_marks_removed_pipe(vec![]), vec![]);
    assert_eq!(
        leading_marks_removed_pipe(vec![
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
            4usize, 0usize, 0usize, 3usize, 10usize, 5usize, 7usize, 10usize, 0usize, 5usize
        ]
    );
    assert_eq!(
        leading_marks_removed_pipe(vec![
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
            3usize, 7usize, 6usize, 4usize, 0usize, 0usize, 3usize, 10usize, 5usize, 7usize,
            10usize, 0usize, 5usize, 3usize
        ]
    );
    assert_eq!(
        leading_marks_removed_pipe(vec!["", "|", "|"]),
        vec![0usize, 0usize, 0usize]
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_974::TRACK_PROOF, 0x52540000 | 974);
}
