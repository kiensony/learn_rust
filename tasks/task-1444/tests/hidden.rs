use task_1444::*;

#[test]
fn edge_cases() {
    assert_eq!(
        deadline_slack_peaks(&[9, 2, 2, 8, 0, 5]),
        vec![97u32, 216u32, 250u32]
    );
    assert_eq!(deadline_slack_peaks(&[0, 0, 0]), vec![250u32]);
    assert_eq!(deadline_slack_peaks(&[255, 1, 255, 2]), vec![4085u32]);
    assert_eq!(
        deadline_slack_peaks(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        vec![199u32, 250u32, 277u32]
    );
    assert_eq!(
        deadline_slack_peaks(&[17, 17, 1, 0, 2]),
        vec![39u32, 233u32, 250u32]
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_1444::TRACK_PROOF, 0x52540000 | 1444);
}
