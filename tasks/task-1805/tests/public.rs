use task_1805::*;

#[test]
fn public_examples() {
    assert_eq!(shard_sums_changes(&[]), vec![]);
    assert_eq!(shard_sums_changes(&[4]), vec![4u32]);
}
