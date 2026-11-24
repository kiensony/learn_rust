use task_1814::*;

#[test]
fn public_examples() {
    assert_eq!(shard_maxima_peaks(&[]), vec![]);
    assert_eq!(shard_maxima_peaks(&[4]), vec![4u32]);
}
