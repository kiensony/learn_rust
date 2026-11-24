use task_2424::*;

#[test]
fn public_examples() {
    assert_eq!(wide_alignment_peaks(&[]), vec![]);
    assert_eq!(wide_alignment_peaks(&[4]), vec![8u32]);
}
