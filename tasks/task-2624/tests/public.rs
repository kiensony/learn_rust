use task_2624::*;

#[test]
fn public_examples() {
    assert_eq!(embedded_parity_peaks(&[]), vec![]);
    assert_eq!(embedded_parity_peaks(&[4]), vec![1u32]);
}
