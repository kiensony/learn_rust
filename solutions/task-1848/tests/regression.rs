use task_1848::*;

#[test]
fn transfer_case() {
    assert_eq!(
        shard_masks_rolling_load(
            &[
                3, 6, 9, 12, 21, 15, 0, 26, 8, 31, 31, 8, 26, 0, 15, 21, 12, 9, 6, 3
            ],
            3
        ),
        vec![77u64, 93u64, 77u64]
    );
}
