use task_2698::*;

#[test]
fn public_examples() {
    assert_eq!(
        lending_checksum_rolling_load(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31], 1),
        vec![0u64, 0u64, 23u64]
    );
    assert_eq!(lending_checksum_rolling_load(&[], 0), vec![]);
}
