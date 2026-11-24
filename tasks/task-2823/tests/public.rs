use task_2823::*;

#[test]
fn public_examples() {
    assert_eq!(service_sharding_bounds(&[]), None);
    assert_eq!(service_sharding_bounds(&[4]), Some((0u32, 0u32)));
}
