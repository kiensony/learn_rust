use task_2512::*;

#[test]
fn public_examples() {
    assert_eq!(abi_flags_total(&[]), 0);
    assert_eq!(abi_flags_total(&[4]), 1);
}
