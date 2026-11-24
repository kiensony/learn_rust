use task_2189::*;

#[test]
fn public_examples() {
    assert_eq!(early_exit_probes_longest_run(&[]), 0);
    assert_eq!(early_exit_probes_longest_run(&[4]), 1);
}
