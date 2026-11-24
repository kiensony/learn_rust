use task_1400::*;

#[test]
fn transfer_case() {
    assert_eq!(
        frame_maxima_replay_mismatch(
            &[2, 7, 3, 1, 4, 0, 3, 9, 9, 2, 2, 9, 9, 3, 0, 4, 1, 3, 7, 2],
            &[7, 4, 0, 9, 9, 4]
        ),
        None
    );
}
