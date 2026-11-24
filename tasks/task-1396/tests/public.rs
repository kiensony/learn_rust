use task_1396::*;

#[test]
fn public_examples() {
    assert_eq!(
        frame_lengths_threshold_crossing(&[2, 7, 3, 1, 4, 0, 3, 9, 9, 2], 0),
        Some((0, 2))
    );
    assert_eq!(frame_lengths_threshold_crossing(&[], 0), None);
}
