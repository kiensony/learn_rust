use task_1808::*;

#[test]
fn public_examples() {
    assert_eq!(shard_sums_steps(&[]), vec![]);
    assert_eq!(shard_sums_steps(&[4]), vec![]);
}
