use task_1293::*;

#[test]
fn edge_cases() {
    assert_eq!(
        quiet_option_positions_bounds("-q -q -- -v"),
        Some((0u32, 1u32))
    );
    assert_eq!(
        quiet_option_positions_bounds("--level=3 data --level=9"),
        None
    );
    assert_eq!(quiet_option_positions_bounds("a --bad --level=x b"), None);
    assert_eq!(
        quiet_option_positions_bounds("input -v -q input --level=255"),
        Some((2u32, 2u32))
    );
    assert_eq!(quiet_option_positions_bounds("-- --level=4 x"), None);
}

#[test]
fn submission_marker() {
    assert_eq!(task_1293::TRACK_PROOF, 0x52540000 | 1293);
}
