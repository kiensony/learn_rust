use task_2526::*;

#[test]
fn public_examples() {
    assert_eq!(abi_signed_decode_descending_steps(&[]), 0);
    assert_eq!(abi_signed_decode_descending_steps(&[4]), 0);
}
