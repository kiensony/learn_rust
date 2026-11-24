use task_2535::*;

#[test]
fn public_examples() {
    assert_eq!(abi_endian_swap_changes(&[]), vec![]);
    assert_eq!(abi_endian_swap_changes(&[4]), vec![1024u32]);
}
