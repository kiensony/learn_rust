use task_1300::*;

#[test]
fn transfer_case() {
    assert_eq!(
        quiet_option_positions_replay_mismatch("-q -q -- -v -q -q -- -v", &[0, 1, 4, 5]),
        None
    );
}
