use task_1396::*;

#[test]
fn transfer_case() {
    assert_eq!(
        frame_lengths_threshold_crossing(
            &[2, 7, 3, 1, 4, 0, 3, 9, 9, 2, 2, 9, 9, 3, 0, 4, 1, 3, 7, 2],
            1
        ),
        Some((0, 2))
    );
}
