use task_1819::*;

#[test]
fn public_examples() {
    assert_eq!(shard_maxima_longest_run(&[]), 0);
    assert_eq!(shard_maxima_longest_run(&[4]), 1);
}
