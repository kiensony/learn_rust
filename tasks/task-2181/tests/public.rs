use task_2181::*;

#[test]
fn public_examples() {
    assert_eq!(early_exit_probes_trace(&[]), vec![]);
    assert_eq!(early_exit_probes_trace(&[4]), vec![0u32]);
}
