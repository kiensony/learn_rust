use task_2523::*;

#[test]
fn public_examples() {
    assert_eq!(abi_signed_decode_bounds(&[]), None);
    assert_eq!(abi_signed_decode_bounds(&[4]), Some((4u32, 4u32)));
}
