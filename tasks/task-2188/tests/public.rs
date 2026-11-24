use task_2188::*;

#[test]
fn public_examples() {
    assert_eq!(early_exit_probes_steps(&[]), vec![]);
    assert_eq!(early_exit_probes_steps(&[4]), vec![]);
}
