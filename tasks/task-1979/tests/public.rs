use task_1979::*;

#[test]
fn public_examples() {
    assert_eq!(atomic_toggle_longest_run(&[]), 0);
    assert_eq!(atomic_toggle_longest_run(&[4]), 1);
}
