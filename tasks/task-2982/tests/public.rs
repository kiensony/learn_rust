use task_2982::*;

#[test]
fn public_examples() {
    assert_eq!(service_wait_total(&[]), 0);
    assert_eq!(service_wait_total(&[4]), 0);
}
