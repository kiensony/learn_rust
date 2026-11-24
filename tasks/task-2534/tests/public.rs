use task_2534::*;

#[test]
fn public_examples() {
    assert_eq!(abi_endian_swap_peaks(&[]), vec![]);
    assert_eq!(abi_endian_swap_peaks(&[4]), vec![1024u32]);
}
