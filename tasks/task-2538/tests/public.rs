use task_2538::*;

#[test]
fn public_examples() {
    assert_eq!(abi_endian_swap_steps(&[]), vec![]);
    assert_eq!(abi_endian_swap_steps(&[4]), vec![]);
}
