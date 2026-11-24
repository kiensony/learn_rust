use task_2384::*;

#[test]
fn public_examples() {
    assert_eq!(rolling_parity_peaks(&[]), vec![]);
    assert_eq!(rolling_parity_peaks(&[4]), vec![4u32]);
}
