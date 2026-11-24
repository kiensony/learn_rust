use task_1950::*;

#[test]
fn transfer_case() {
    assert_eq!(
        locked_batch_peaks_replay_mismatch(
            &[
                3, 6, 9, 12, 21, 15, 0, 26, 8, 31, 31, 8, 26, 0, 15, 21, 12, 9, 6, 3
            ],
            &[9, 21, 26, 31, 26, 21, 6]
        ),
        None
    );
}
