use task_2187::*;

#[test]
fn public_examples() {
    assert_eq!(early_exit_probes_distinct(&[]), vec![]);
    assert_eq!(early_exit_probes_distinct(&[4]), vec![0u32]);
}
