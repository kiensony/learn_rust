use task_1294::*;

#[test]
fn edge_cases() {
    assert_eq!(
        quiet_option_positions_peaks("-q -q -- -v"),
        vec![0u32, 1u32]
    );
    assert_eq!(
        quiet_option_positions_peaks("--level=3 data --level=9"),
        vec![]
    );
    assert_eq!(quiet_option_positions_peaks("a --bad --level=x b"), vec![]);
    assert_eq!(
        quiet_option_positions_peaks("input -v -q input --level=255"),
        vec![2u32]
    );
    assert_eq!(quiet_option_positions_peaks("-- --level=4 x"), vec![]);
}

#[test]
fn submission_marker() {
    assert_eq!(task_1294::TRACK_PROOF, 0x52540000 | 1294);
}
