use task_2509::*;

#[test]
fn public_examples() {
    assert_eq!(abi_unsigned_widening_longest_run(&[]), 0);
    assert_eq!(abi_unsigned_widening_longest_run(&[4]), 1);
}
