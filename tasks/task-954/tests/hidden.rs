use task_954::*;

#[test]
fn edge_cases() {
    assert_eq!(closing_field_colon(vec![]), vec![]);
    assert_eq!(
        closing_field_colon(vec![":", "cat", "::one:猫:ab", "aa:bb", "ok:🐈"]),
        vec![0usize, 3usize, 2usize, 2usize, 4usize]
    );
    assert_eq!(
        closing_field_colon(vec![
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
            3usize, 1usize, 0usize, 0usize, 3usize, 2usize, 2usize, 4usize, 3usize
        ]
    );
    assert_eq!(
        closing_field_colon(vec!["", ":", ":"]),
        vec![0usize, 0usize, 0usize]
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_954::TRACK_PROOF, 0x52540000 | 954);
}
