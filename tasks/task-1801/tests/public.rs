use task_1801::*;

#[test]
fn public_examples() {
    assert_eq!(shard_sums_trace(&[]), vec![]);
    assert_eq!(shard_sums_trace(&[4]), vec![4u32]);
}
