use task_2183::*;

#[test]
fn public_examples() {
    assert_eq!(early_exit_probes_bounds(&[]), None);
    assert_eq!(early_exit_probes_bounds(&[4]), Some((0u32, 0u32)));
}
