use task_2634::*;

#[test]
fn public_examples() {
    assert_eq!(embedded_saturation_peaks(&[]), vec![]);
    assert_eq!(embedded_saturation_peaks(&[4]), vec![8u32]);
}
