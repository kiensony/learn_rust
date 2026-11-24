use task_2511::*;

#[test]
fn public_examples() {
    assert_eq!(abi_flags_trace(&[]), vec![]);
    assert_eq!(abi_flags_trace(&[4]), vec![1u32]);
}
