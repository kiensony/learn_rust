use task_1993::*;

#[test]
fn public_examples() {
    assert_eq!(atomic_exchange_bounds(&[]), None);
    assert_eq!(atomic_exchange_bounds(&[4]), Some((0u32, 0u32)));
}
