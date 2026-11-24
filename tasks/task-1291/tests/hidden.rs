use task_1291::*;

#[test]
fn edge_cases() {
    assert_eq!(
        quiet_option_positions_trace("-q -q -- -v"),
        vec![0u32, 1u32]
    );
    assert_eq!(
        quiet_option_positions_trace("--level=3 data --level=9"),
        vec![]
    );
    assert_eq!(quiet_option_positions_trace("a --bad --level=x b"), vec![]);
    assert_eq!(
        quiet_option_positions_trace("input -v -q input --level=255"),
        vec![2u32]
    );
    assert_eq!(quiet_option_positions_trace("-- --level=4 x"), vec![]);
}

#[test]
fn submission_marker() {
    assert_eq!(task_1291::TRACK_PROOF, 0x52540000 | 1291);
}
