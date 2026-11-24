use task_2502::*;

#[test]
fn public_examples() {
    assert_eq!(abi_unsigned_widening_total(&[]), 0);
    assert_eq!(abi_unsigned_widening_total(&[4]), 4);
}
