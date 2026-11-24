use task_2987::*;

#[test]
fn public_examples() {
    assert_eq!(service_wait_distinct(&[]), vec![]);
    assert_eq!(service_wait_distinct(&[4]), vec![]);
}
