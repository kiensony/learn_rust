use task_1955::*;

#[test]
fn public_examples() {
    assert_eq!(atomic_running_total_changes(&[]), vec![]);
    assert_eq!(atomic_running_total_changes(&[4]), vec![0u32]);
}
