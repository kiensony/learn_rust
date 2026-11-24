use task_2501::*;

#[test]
fn public_examples() {
    assert_eq!(abi_unsigned_widening_trace(&[]), vec![]);
    assert_eq!(abi_unsigned_widening_trace(&[4]), vec![4u32]);
}
