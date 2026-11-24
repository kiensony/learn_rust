use task_2444::*;

#[test]
fn public_examples() {
    assert_eq!(page_alignment_peaks(&[]), vec![]);
    assert_eq!(page_alignment_peaks(&[4]), vec![32u32]);
}
