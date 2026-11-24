use task_2519::*;

#[test]
fn public_examples() {
    assert_eq!(abi_flags_longest_run(&[]), 0);
    assert_eq!(abi_flags_longest_run(&[4]), 1);
}
