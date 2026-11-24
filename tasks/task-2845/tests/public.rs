use task_2845::*;

#[test]
fn public_examples() {
    assert_eq!(service_status_changes(&[]), vec![]);
    assert_eq!(service_status_changes(&[4]), vec![0u32]);
}
