use task_2507::*;

#[test]
fn public_examples() {
    assert_eq!(abi_unsigned_widening_distinct(&[]), vec![]);
    assert_eq!(abi_unsigned_widening_distinct(&[4]), vec![4u32]);
}
