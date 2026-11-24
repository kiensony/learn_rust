use task_1950::*;

#[test]
fn public_examples() {
    assert_eq!(
        locked_batch_peaks_replay_mismatch(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31], &[9, 21, 26]),
        Some(3)
    );
    assert_eq!(locked_batch_peaks_replay_mismatch(&[], &[]), None);
}
