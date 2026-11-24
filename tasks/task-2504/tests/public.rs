use task_2504::*;

#[test]
fn public_examples() {
    assert_eq!(abi_unsigned_widening_peaks(&[]), vec![]);
    assert_eq!(abi_unsigned_widening_peaks(&[4]), vec![4u32]);
}
