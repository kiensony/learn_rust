use task_2825::*;

#[test]
fn public_examples() {
    assert_eq!(service_sharding_changes(&[]), vec![]);
    assert_eq!(service_sharding_changes(&[4]), vec![0u32]);
}
