use task_2410::*;

#[test]
fn public_examples() {
    assert_eq!(header_alignment_reverse(&[]), vec![]);
    assert_eq!(header_alignment_reverse(&[4]), vec![4u32]);
}
