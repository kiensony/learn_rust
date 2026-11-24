use task_2524::*;

#[test]
fn public_examples() {
    assert_eq!(abi_signed_decode_peaks(&[]), vec![]);
    assert_eq!(abi_signed_decode_peaks(&[4]), vec![4u32]);
}
