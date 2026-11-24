use task_2182::*;

#[test]
fn edge_cases() {
    assert_eq!(early_exit_probes_total(&[9, 2, 2, 8, 0, 5]), 5);
    assert_eq!(early_exit_probes_total(&[0, 0, 0]), 3);
    assert_eq!(early_exit_probes_total(&[255, 1, 255, 2]), 4);
    assert_eq!(
        early_exit_probes_total(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        35
    );
    assert_eq!(early_exit_probes_total(&[17, 17, 1, 0, 2]), 4);
}

#[test]
fn submission_marker() {
    assert_eq!(task_2182::TRACK_PROOF, 0x52540000 | 2182);
}
