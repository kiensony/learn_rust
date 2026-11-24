use task_1250::*;

#[test]
fn public_examples() {
    assert_eq!(
        limited_reads_replay_mismatch(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31], &[3, 6, 9]),
        Some(3)
    );
    assert_eq!(limited_reads_replay_mismatch(&[], &[]), None);
}
