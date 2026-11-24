use task_1300::*;

#[test]
fn edge_cases() {
    assert_eq!(quiet_option_positions_replay_mismatch("only", &[]), None);
    assert_eq!(
        quiet_option_positions_replay_mismatch("-q -q -- -v", &[0, 1, 777]),
        Some(2)
    );
    assert_eq!(
        quiet_option_positions_replay_mismatch("--level=3 data --level=9", &[]),
        None
    );
    assert_eq!(
        quiet_option_positions_replay_mismatch("a --bad --level=x b", &[9999]),
        Some(0)
    );
    assert_eq!(
        quiet_option_positions_replay_mismatch("input -v -q input --level=255", &[2]),
        None
    );
    assert_eq!(
        quiet_option_positions_replay_mismatch("-- --level=4 x", &[]),
        None
    );
    assert_eq!(
        quiet_option_positions_replay_mismatch("-q -q -- -v", &[0, 1, 1]),
        Some(2)
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_1300::TRACK_PROOF, 0x52540000 | 1300);
}
