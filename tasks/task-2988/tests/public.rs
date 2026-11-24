use task_2988::*;

#[test]
fn public_examples() {
    assert_eq!(service_wait_steps(&[]), vec![]);
    assert_eq!(service_wait_steps(&[4]), vec![]);
}
