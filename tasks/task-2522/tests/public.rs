use task_2522::*;

#[test]
fn public_examples() {
    assert_eq!(abi_signed_decode_total(&[]), 0);
    assert_eq!(abi_signed_decode_total(&[4]), 4);
}
