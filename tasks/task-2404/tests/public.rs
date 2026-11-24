use task_2404::*;

#[test]
fn public_examples() {
    assert_eq!(header_alignment_peaks(&[]), vec![]);
    assert_eq!(header_alignment_peaks(&[4]), vec![4u32]);
}
