use task_1450::*;

#[test]
fn edge_cases() {
    assert_eq!(deadline_slack_replay_mismatch(&[4], &[182]), None);
    assert_eq!(
        deadline_slack_replay_mismatch(&[9, 2, 2, 8, 0, 5], &[97, 216, 216, 114, 250, 165, 777]),
        Some(6)
    );
    assert_eq!(
        deadline_slack_replay_mismatch(&[0, 0, 0], &[250, 250]),
        Some(2)
    );
    assert_eq!(
        deadline_slack_replay_mismatch(&[255, 1, 255, 2], &[9999]),
        Some(0)
    );
    assert_eq!(
        deadline_slack_replay_mismatch(
            &[3, 6, 9, 12, 21, 15, 0, 26, 8, 31],
            &[277, 114, 192, 250, 5, 107, 46, 97, 148, 199]
        ),
        Some(0)
    );
    assert_eq!(
        deadline_slack_replay_mismatch(&[17, 17, 1, 0, 2], &[39, 39, 233, 250, 216]),
        None
    );
    assert_eq!(
        deadline_slack_replay_mismatch(
            &[3, 6, 9, 12, 21, 15, 0, 26, 8, 31],
            &[199, 148, 97, 46, 107, 5, 250, 192, 114, 277, 1]
        ),
        Some(10)
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_1450::TRACK_PROOF, 0x52540000 | 1450);
}
