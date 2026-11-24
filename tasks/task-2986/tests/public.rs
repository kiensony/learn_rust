use task_2986::*;

#[test]
fn public_examples() {
    assert_eq!(service_wait_descending_steps(&[]), 0);
    assert_eq!(service_wait_descending_steps(&[4]), 0);
}
