use task_2537::*;

#[test]
fn public_examples() {
    assert_eq!(abi_endian_swap_distinct(&[]), vec![]);
    assert_eq!(abi_endian_swap_distinct(&[4]), vec![1024u32]);
}
