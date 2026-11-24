use task_1295::*;

#[test]
fn edge_cases() {
    assert_eq!(
        quiet_option_positions_changes("-q -q -- -v"),
        vec![0u32, 1u32]
    );
    assert_eq!(
        quiet_option_positions_changes("--level=3 data --level=9"),
        vec![]
    );
    assert_eq!(
        quiet_option_positions_changes("a --bad --level=x b"),
        vec![]
    );
    assert_eq!(
        quiet_option_positions_changes("input -v -q input --level=255"),
        vec![2u32]
    );
    assert_eq!(quiet_option_positions_changes("-- --level=4 x"), vec![]);
}

#[test]
fn submission_marker() {
    assert_eq!(task_1295::TRACK_PROOF, 0x52540000 | 1295);
}
