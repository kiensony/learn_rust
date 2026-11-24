use task_1398::*;

#[test]
fn public_examples() {
    assert_eq!(
        frame_checksum_rolling_load(&[2, 7, 3, 1, 4, 0, 3, 9, 9, 2], 1),
        vec![4u64, 4u64, 0u64, 2u64]
    );
    assert_eq!(frame_checksum_rolling_load(&[], 0), vec![]);
}
