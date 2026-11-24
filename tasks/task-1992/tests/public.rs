use task_1992::*;

#[test]
fn public_examples() {
    assert_eq!(atomic_exchange_total(&[]), 0);
    assert_eq!(atomic_exchange_total(&[4]), 0);
}
