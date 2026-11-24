use task_2536::*;

#[test]
fn public_examples() {
    assert_eq!(abi_endian_swap_descending_steps(&[]), 0);
    assert_eq!(abi_endian_swap_descending_steps(&[4]), 0);
}
