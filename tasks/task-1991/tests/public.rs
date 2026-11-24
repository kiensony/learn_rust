use task_1991::*;

#[test]
fn public_examples() {
    assert_eq!(atomic_exchange_trace(&[]), vec![]);
    assert_eq!(atomic_exchange_trace(&[4]), vec![0u32]);
}
