use task_2822::*;

#[test]
fn public_examples() {
    assert_eq!(service_sharding_total(&[]), 0);
    assert_eq!(service_sharding_total(&[4]), 0);
}
