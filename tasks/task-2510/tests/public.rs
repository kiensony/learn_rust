use task_2510::*;

#[test]
fn public_examples() {
    assert_eq!(abi_unsigned_widening_reverse(&[]), vec![]);
    assert_eq!(abi_unsigned_widening_reverse(&[4]), vec![4u32]);
}
