use task_2184::*;

#[test]
fn public_examples() {
    assert_eq!(early_exit_probes_peaks(&[]), vec![]);
    assert_eq!(early_exit_probes_peaks(&[4]), vec![0u32]);
}
