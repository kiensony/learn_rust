use task_2525::*;

#[test]
fn public_examples() {
    assert_eq!(abi_signed_decode_changes(&[]), vec![]);
    assert_eq!(abi_signed_decode_changes(&[4]), vec![4u32]);
}
