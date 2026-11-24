use task_1804::*;

#[test]
fn public_examples() {
    assert_eq!(shard_sums_peaks(&[]), vec![]);
    assert_eq!(shard_sums_peaks(&[4]), vec![4u32]);
}
