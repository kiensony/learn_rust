use task_2829::*;

#[test]
fn public_examples() {
    assert_eq!(service_sharding_longest_run(&[]), 0);
    assert_eq!(service_sharding_longest_run(&[4]), 1);
}
