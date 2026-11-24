use task_1818::*;

#[test]
fn public_examples() {
    assert_eq!(shard_maxima_steps(&[]), vec![]);
    assert_eq!(shard_maxima_steps(&[4]), vec![]);
}
