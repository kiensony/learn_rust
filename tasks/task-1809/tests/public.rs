use task_1809::*;

#[test]
fn public_examples() {
    assert_eq!(shard_sums_longest_run(&[]), 0);
    assert_eq!(shard_sums_longest_run(&[4]), 1);
}
