use task_2500::*;

#[test]
fn public_examples() {
    assert_eq!(
        interior_raw_reads_replay_mismatch(
            &[3, 6, 9, 12, 21, 15, 0, 26, 8, 31],
            &[6, 9, 12, 21, 15, 0, 26]
        ),
        Some(7)
    );
    assert_eq!(interior_raw_reads_replay_mismatch(&[], &[]), None);
}
