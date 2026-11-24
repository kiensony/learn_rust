use task_2532::*;

#[test]
fn public_examples() {
    assert_eq!(abi_endian_swap_total(&[]), 0);
    assert_eq!(abi_endian_swap_total(&[4]), 1024);
}
