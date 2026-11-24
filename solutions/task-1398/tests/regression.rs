use task_1398::*;

#[test]
fn transfer_case() {
    assert_eq!(
        frame_checksum_rolling_load(
            &[2, 7, 3, 1, 4, 0, 3, 9, 9, 2, 2, 9, 9, 3, 0, 4, 1, 3, 7, 2],
            3
        ),
        vec![8u64, 6u64, 2u64, 7u64]
    );
}
