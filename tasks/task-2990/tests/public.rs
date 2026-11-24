use task_2990::*;

#[test]
fn public_examples() {
    assert_eq!(service_wait_reverse(&[]), vec![]);
    assert_eq!(service_wait_reverse(&[4]), vec![]);
}
