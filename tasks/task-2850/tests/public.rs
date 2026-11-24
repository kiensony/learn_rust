use task_2850::*;

#[test]
fn public_examples() {
    assert_eq!(
        service_status_replay_mismatch(
            &[3, 6, 9, 12, 21, 15, 0, 26, 8, 31],
            &[0, 0, 0, 0, 0, 0, 0, 0, 0]
        ),
        Some(9)
    );
    assert_eq!(service_status_replay_mismatch(&[], &[]), None);
}
