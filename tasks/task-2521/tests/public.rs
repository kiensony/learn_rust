use task_2521::*;

#[test]
fn public_examples() {
    assert_eq!(abi_signed_decode_trace(&[]), vec![]);
    assert_eq!(abi_signed_decode_trace(&[4]), vec![4u32]);
}
