use task_2518::*;

#[test]
fn public_examples() {
    assert_eq!(abi_flags_steps(&[]), vec![]);
    assert_eq!(abi_flags_steps(&[4]), vec![]);
}
