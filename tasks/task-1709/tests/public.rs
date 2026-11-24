use task_1709::*;

#[test]
fn public_examples() {
    assert_eq!(nibble_macro_longest_run(&[]), 0);
    assert_eq!(nibble_macro_longest_run(&[4]), 1);
}
