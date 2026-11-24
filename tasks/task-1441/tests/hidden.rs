use task_1441::*;

#[test]
fn edge_cases() {
    assert_eq!(
        deadline_slack_trace(&[9, 2, 2, 8, 0, 5]),
        vec![97u32, 216u32, 216u32, 114u32, 250u32, 165u32]
    );
    assert_eq!(
        deadline_slack_trace(&[0, 0, 0]),
        vec![250u32, 250u32, 250u32]
    );
    assert_eq!(
        deadline_slack_trace(&[255, 1, 255, 2]),
        vec![4085u32, 233u32, 4085u32, 216u32]
    );
    assert_eq!(
        deadline_slack_trace(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        vec![
            199u32, 148u32, 97u32, 46u32, 107u32, 5u32, 250u32, 192u32, 114u32, 277u32
        ]
    );
    assert_eq!(
        deadline_slack_trace(&[17, 17, 1, 0, 2]),
        vec![39u32, 39u32, 233u32, 250u32, 216u32]
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_1441::TRACK_PROOF, 0x52540000 | 1441);
}
