use task_2516::*;

#[test]
fn public_examples() {
    assert_eq!(abi_flags_descending_steps(&[]), 0);
    assert_eq!(abi_flags_descending_steps(&[4]), 0);
}
