use task_1806::*;

#[test]
fn public_examples() {
    assert_eq!(shard_sums_descending_steps(&[]), 0);
    assert_eq!(shard_sums_descending_steps(&[4]), 0);
}
