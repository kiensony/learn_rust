use task_2527::*;

#[test]
fn public_examples() {
    assert_eq!(abi_signed_decode_distinct(&[]), vec![]);
    assert_eq!(abi_signed_decode_distinct(&[4]), vec![4u32]);
}
