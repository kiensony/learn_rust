use task_1810::*;

#[test]
fn public_examples() {
    assert_eq!(shard_sums_reverse(&[]), vec![]);
    assert_eq!(shard_sums_reverse(&[4]), vec![4u32]);
}
