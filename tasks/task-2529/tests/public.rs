use task_2529::*;

#[test]
fn public_examples() {
    assert_eq!(abi_signed_decode_longest_run(&[]), 0);
    assert_eq!(abi_signed_decode_longest_run(&[4]), 1);
}
