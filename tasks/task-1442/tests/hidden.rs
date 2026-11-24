use task_1442::*;

#[test]
fn edge_cases() {
    assert_eq!(deadline_slack_total(&[9, 2, 2, 8, 0, 5]), 1058);
    assert_eq!(deadline_slack_total(&[0, 0, 0]), 750);
    assert_eq!(deadline_slack_total(&[255, 1, 255, 2]), 8619);
    assert_eq!(
        deadline_slack_total(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        1435
    );
    assert_eq!(deadline_slack_total(&[17, 17, 1, 0, 2]), 777);
}

#[test]
fn submission_marker() {
    assert_eq!(task_1442::TRACK_PROOF, 0x52540000 | 1442);
}
