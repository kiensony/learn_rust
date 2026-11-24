use task_2530::*;

#[test]
fn public_examples() {
    assert_eq!(abi_signed_decode_reverse(&[]), vec![]);
    assert_eq!(abi_signed_decode_reverse(&[4]), vec![4u32]);
}
