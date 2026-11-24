use task_1995::*;

#[test]
fn public_examples() {
    assert_eq!(atomic_exchange_changes(&[]), vec![]);
    assert_eq!(atomic_exchange_changes(&[4]), vec![0u32]);
}
