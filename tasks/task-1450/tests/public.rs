use task_1450::*;

#[test]
fn public_examples() {
    assert_eq!(
        deadline_slack_replay_mismatch(
            &[3, 6, 9, 12, 21, 15, 0, 26, 8, 31],
            &[199, 148, 97, 46, 107, 5, 250, 192, 114]
        ),
        Some(9)
    );
    assert_eq!(deadline_slack_replay_mismatch(&[], &[]), None);
}
