use task_953::*;

#[test]
fn edge_cases() {
    assert_eq!(leading_field_colon(vec![]), vec![]);
    assert_eq!(
        leading_field_colon(vec![":", "cat", "::one:猫:ab", "aa:bb", "ok:🐈"]),
        vec![0usize, 3usize, 0usize, 2usize, 2usize]
    );
    assert_eq!(
        leading_field_colon(vec![
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
            2usize, 0usize, 0usize, 0usize, 3usize, 0usize, 2usize, 2usize, 2usize
        ]
    );
    assert_eq!(
        leading_field_colon(vec!["", ":", ":"]),
        vec![0usize, 0usize, 0usize]
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_953::TRACK_PROOF, 0x52540000 | 953);
}
