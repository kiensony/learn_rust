use task_2827::*;

#[test]
fn public_examples() {
    assert_eq!(service_sharding_distinct(&[]), vec![]);
    assert_eq!(service_sharding_distinct(&[4]), vec![0u32]);
}
