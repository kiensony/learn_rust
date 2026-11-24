use task_2533::*;

#[test]
fn public_examples() {
    assert_eq!(abi_endian_swap_bounds(&[]), None);
    assert_eq!(abi_endian_swap_bounds(&[4]), Some((1024u32, 1024u32)));
}
