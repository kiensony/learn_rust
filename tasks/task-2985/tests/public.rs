use task_2985::*;

#[test]
fn public_examples() {
    assert_eq!(service_wait_changes(&[]), vec![]);
    assert_eq!(service_wait_changes(&[4]), vec![]);
}
