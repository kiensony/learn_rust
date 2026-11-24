use task_2540::*;

#[test]
fn public_examples() {
    assert_eq!(abi_endian_swap_reverse(&[]), vec![]);
    assert_eq!(abi_endian_swap_reverse(&[4]), vec![1024u32]);
}
