use task_2508::*;

#[test]
fn public_examples() {
    assert_eq!(abi_unsigned_widening_steps(&[]), vec![]);
    assert_eq!(abi_unsigned_widening_steps(&[4]), vec![]);
}
