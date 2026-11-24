use task_2983::*;

#[test]
fn public_examples() {
    assert_eq!(service_wait_bounds(&[]), None);
    assert_eq!(service_wait_bounds(&[4]), None);
}
