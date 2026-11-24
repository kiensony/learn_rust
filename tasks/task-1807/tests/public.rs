use task_1807::*;

#[test]
fn public_examples() {
    assert_eq!(shard_sums_distinct(&[]), vec![]);
    assert_eq!(shard_sums_distinct(&[4]), vec![4u32]);
}
