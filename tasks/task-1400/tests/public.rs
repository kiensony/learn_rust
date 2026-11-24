use task_1400::*;

#[test]
fn public_examples() {
    assert_eq!(
        frame_maxima_replay_mismatch(&[2, 7, 3, 1, 4, 0, 3, 9, 9, 2], &[7, 4, 0]),
        Some(3)
    );
    assert_eq!(frame_maxima_replay_mismatch(&[], &[]), None);
}
