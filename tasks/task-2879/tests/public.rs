use task_2879::*;

#[test]
fn public_examples() {
    assert_eq!(flag_register_longest_run(&[]), 0);
    assert_eq!(flag_register_longest_run(&[4]), 0);
}
