use task_2505::*;

#[test]
fn public_examples() {
    assert_eq!(abi_unsigned_widening_changes(&[]), vec![]);
    assert_eq!(abi_unsigned_widening_changes(&[4]), vec![4u32]);
}
