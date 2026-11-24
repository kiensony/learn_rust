use task_2619::*;

#[test]
fn public_examples() {
    assert_eq!(embedded_flags_longest_run(&[]), 0);
    assert_eq!(embedded_flags_longest_run(&[4]), 1);
}
