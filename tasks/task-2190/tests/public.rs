use task_2190::*;

#[test]
fn public_examples() {
    assert_eq!(early_exit_probes_reverse(&[]), vec![]);
    assert_eq!(early_exit_probes_reverse(&[4]), vec![0u32]);
}
