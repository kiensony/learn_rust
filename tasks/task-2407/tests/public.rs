use task_2407::*;

#[test]
fn public_examples() {
    assert_eq!(header_alignment_distinct(&[]), vec![]);
    assert_eq!(header_alignment_distinct(&[4]), vec![4u32]);
}
