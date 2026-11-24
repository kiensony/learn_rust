use task_2700::*;

#[test]
fn public_examples() {
    assert_eq!(
        universal_slice_width_replay_mismatch(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31], &[18]),
        Some(1)
    );
    assert_eq!(universal_slice_width_replay_mismatch(&[], &[]), None);
}
