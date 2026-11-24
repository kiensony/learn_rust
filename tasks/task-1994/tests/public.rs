use task_1994::*;

#[test]
fn public_examples() {
    assert_eq!(atomic_exchange_peaks(&[]), vec![]);
    assert_eq!(atomic_exchange_peaks(&[4]), vec![0u32]);
}
