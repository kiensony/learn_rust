use task_1813::*;

#[test]
fn public_examples() {
    assert_eq!(shard_maxima_bounds(&[]), None);
    assert_eq!(shard_maxima_bounds(&[4]), Some((4u32, 4u32)));
}
