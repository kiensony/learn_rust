use task_2531::*;

#[test]
fn public_examples() {
    assert_eq!(abi_endian_swap_trace(&[]), vec![]);
    assert_eq!(abi_endian_swap_trace(&[4]), vec![1024u32]);
}
