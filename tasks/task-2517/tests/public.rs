use task_2517::*;

#[test]
fn public_examples() {
    assert_eq!(abi_flags_distinct(&[]), vec![]);
    assert_eq!(abi_flags_distinct(&[4]), vec![1u32]);
}
