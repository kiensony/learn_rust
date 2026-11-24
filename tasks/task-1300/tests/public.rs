use task_1300::*;

#[test]
fn public_examples() {
    assert_eq!(
        quiet_option_positions_replay_mismatch("-q -q -- -v", &[0]),
        Some(1)
    );
    assert_eq!(quiet_option_positions_replay_mismatch("", &[]), None);
}
