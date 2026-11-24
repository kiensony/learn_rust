use task_2629::*;

#[test]
fn public_examples() {
    assert_eq!(embedded_parity_longest_run(&[]), 0);
    assert_eq!(embedded_parity_longest_run(&[4]), 1);
}
