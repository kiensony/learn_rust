use task_1802::*;

#[test]
fn public_examples() {
    assert_eq!(shard_sums_total(&[]), 0);
    assert_eq!(shard_sums_total(&[4]), 4);
}
