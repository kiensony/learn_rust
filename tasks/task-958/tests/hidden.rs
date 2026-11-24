use task_958::*;

#[test]
fn edge_cases() {
    assert_eq!(prefix_before_final_mark_colon(vec![]), vec![]);
    assert_eq!(
        prefix_before_final_mark_colon(vec![":", "cat", "::one:猫:ab", "aa:bb", "ok:🐈"]),
        vec![0usize, 0usize, 9usize, 2usize, 2usize]
    );
    assert_eq!(
        prefix_before_final_mark_colon(vec![
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
            2usize, 3usize, 0usize, 0usize, 0usize, 9usize, 2usize, 2usize, 2usize
        ]
    );
    assert_eq!(
        prefix_before_final_mark_colon(vec!["", ":", ":"]),
        vec![0usize, 0usize, 0usize]
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_958::TRACK_PROOF, 0x52540000 | 958);
}
