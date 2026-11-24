use task_2694::*;

#[test]
fn public_examples() {
    assert_eq!(universal_slice_width_peaks(&[]), vec![]);
    assert_eq!(universal_slice_width_peaks(&[4]), vec![0u32]);
}
