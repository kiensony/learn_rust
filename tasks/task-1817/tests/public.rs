use task_1817::*;

#[test]
fn public_examples() {
    assert_eq!(shard_maxima_distinct(&[]), vec![]);
    assert_eq!(shard_maxima_distinct(&[4]), vec![4u32]);
}
