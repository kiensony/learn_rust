use task_1414::*;

#[test]
fn public_examples() {
    assert_eq!(elapsed_seconds_peaks(&[]), vec![]);
    assert_eq!(elapsed_seconds_peaks(&[4]), vec![0u32]);
}
