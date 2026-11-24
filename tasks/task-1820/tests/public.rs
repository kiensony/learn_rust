use task_1820::*;

#[test]
fn public_examples() {
    assert_eq!(shard_maxima_reverse(&[]), vec![]);
    assert_eq!(shard_maxima_reverse(&[4]), vec![4u32]);
}
