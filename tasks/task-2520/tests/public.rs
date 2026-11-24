use task_2520::*;

#[test]
fn public_examples() {
    assert_eq!(abi_flags_reverse(&[]), vec![]);
    assert_eq!(abi_flags_reverse(&[4]), vec![1u32]);
}
