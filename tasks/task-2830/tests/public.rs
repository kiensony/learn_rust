use task_2830::*;

#[test]
fn public_examples() {
    assert_eq!(service_sharding_reverse(&[]), vec![]);
    assert_eq!(service_sharding_reverse(&[4]), vec![0u32]);
}
