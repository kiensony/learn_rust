use task_2826::*;

#[test]
fn public_examples() {
    assert_eq!(service_sharding_descending_steps(&[]), 0);
    assert_eq!(service_sharding_descending_steps(&[4]), 0);
}
