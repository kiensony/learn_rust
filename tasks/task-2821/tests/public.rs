use task_2821::*;

#[test]
fn public_examples() {
    assert_eq!(service_sharding_trace(&[]), vec![]);
    assert_eq!(service_sharding_trace(&[4]), vec![0u32]);
}
