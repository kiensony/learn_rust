use task_1292::*;

#[test]
fn edge_cases() {
    assert_eq!(quiet_option_positions_total("-q -q -- -v"), 1);
    assert_eq!(quiet_option_positions_total("--level=3 data --level=9"), 0);
    assert_eq!(quiet_option_positions_total("a --bad --level=x b"), 0);
    assert_eq!(
        quiet_option_positions_total("input -v -q input --level=255"),
        2
    );
    assert_eq!(quiet_option_positions_total("-- --level=4 x"), 0);
}

#[test]
fn submission_marker() {
    assert_eq!(task_1292::TRACK_PROOF, 0x52540000 | 1292);
}
