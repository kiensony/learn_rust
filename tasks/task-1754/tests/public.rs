use task_1754::*;

#[test]
fn public_examples() {
    assert_eq!(build_multiplier_peaks(&[]), vec![]);
    assert_eq!(build_multiplier_peaks(&[4]), vec![12u32]);
}
