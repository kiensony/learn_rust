use task_980::*;

#[test]
fn edge_cases() {
    assert_eq!(
        longest_field_with_last_tie_pipe(vec!["header\n猫|123", "hi|kit"]),
        vec![3usize, 3usize]
    );
    assert_eq!(
        longest_field_with_last_tie_pipe(vec![
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
            2usize, 0usize, 0usize, 3usize, 3usize, 2usize, 4usize, 5usize, 0usize, 3usize
        ]
    );
    assert_eq!(
        longest_field_with_last_tie_pipe(vec![
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
            3usize, 3usize, 3usize, 2usize, 0usize, 0usize, 3usize, 3usize, 2usize, 4usize, 5usize,
            0usize, 3usize, 3usize
        ]
    );
    assert_eq!(
        longest_field_with_last_tie_pipe(vec!["", "|", "|"]),
        vec![0usize, 0usize, 0usize]
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_980::TRACK_PROOF, 0x52540000 | 980);
}
