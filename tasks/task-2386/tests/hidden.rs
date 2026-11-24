use task_2386::*;

#[test]
fn edge_cases() {
    assert_eq!(rolling_parity_descending_steps(&[9, 2, 2, 8, 0, 5]), 2);
    assert_eq!(rolling_parity_descending_steps(&[0, 0, 0]), 0);
    assert_eq!(rolling_parity_descending_steps(&[255, 1, 255, 2]), 2);
    assert_eq!(
        rolling_parity_descending_steps(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        2
    );
    assert_eq!(rolling_parity_descending_steps(&[17, 17, 1, 0, 2]), 1);
}

#[test]
fn submission_marker() {
    assert_eq!(task_2386::TRACK_PROOF, 0x52540000 | 2386);
}
