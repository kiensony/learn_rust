use task_2506::*;

#[test]
fn public_examples() {
    assert_eq!(abi_unsigned_widening_descending_steps(&[]), 0);
    assert_eq!(abi_unsigned_widening_descending_steps(&[4]), 0);
}
