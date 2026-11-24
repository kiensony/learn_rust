use task_2185::*;

#[test]
fn public_examples() {
    assert_eq!(early_exit_probes_changes(&[]), vec![]);
    assert_eq!(early_exit_probes_changes(&[4]), vec![0u32]);
}
