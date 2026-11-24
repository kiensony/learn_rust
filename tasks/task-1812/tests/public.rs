use task_1812::*;

#[test]
fn public_examples() {
    assert_eq!(shard_maxima_total(&[]), 0);
    assert_eq!(shard_maxima_total(&[4]), 4);
}
