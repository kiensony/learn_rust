use task_977::*;

#[test]
fn edge_cases() {
    assert_eq!(remainder_after_first_mark_pipe(vec![]), vec![]);
    assert_eq!(
        remainder_after_first_mark_pipe(vec![
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
            4usize, 0usize, 0usize, 0usize, 11usize, 2usize, 4usize, 5usize, 0usize, 1usize
        ]
    );
    assert_eq!(
        remainder_after_first_mark_pipe(vec![
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
            0usize, 3usize, 3usize, 4usize, 0usize, 0usize, 0usize, 11usize, 2usize, 4usize,
            5usize, 0usize, 1usize, 0usize
        ]
    );
    assert_eq!(
        remainder_after_first_mark_pipe(vec!["", "|", "|"]),
        vec![0usize, 0usize, 0usize]
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_977::TRACK_PROOF, 0x52540000 | 977);
}
