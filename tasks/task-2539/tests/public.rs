use task_2539::*;

#[test]
fn public_examples() {
    assert_eq!(abi_endian_swap_longest_run(&[]), 0);
    assert_eq!(abi_endian_swap_longest_run(&[4]), 1);
}
