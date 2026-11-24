use task_2186::*;

#[test]
fn public_examples() {
    assert_eq!(early_exit_probes_descending_steps(&[]), 0);
    assert_eq!(early_exit_probes_descending_steps(&[4]), 0);
}
