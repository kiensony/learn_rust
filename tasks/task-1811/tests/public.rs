use task_1811::*;

#[test]
fn public_examples() {
    assert_eq!(shard_maxima_trace(&[]), vec![]);
    assert_eq!(shard_maxima_trace(&[4]), vec![4u32]);
}
