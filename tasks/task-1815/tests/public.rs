use task_1815::*;

#[test]
fn public_examples() {
    assert_eq!(shard_maxima_changes(&[]), vec![]);
    assert_eq!(shard_maxima_changes(&[4]), vec![4u32]);
}
