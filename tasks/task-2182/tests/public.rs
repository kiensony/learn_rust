use task_2182::*;

#[test]
fn public_examples() {
    assert_eq!(early_exit_probes_total(&[]), 0);
    assert_eq!(early_exit_probes_total(&[4]), 0);
}
