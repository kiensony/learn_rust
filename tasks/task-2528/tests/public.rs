use task_2528::*;

#[test]
fn public_examples() {
    assert_eq!(abi_signed_decode_steps(&[]), vec![]);
    assert_eq!(abi_signed_decode_steps(&[4]), vec![]);
}
