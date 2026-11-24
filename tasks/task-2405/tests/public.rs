use task_2405::*;

#[test]
fn public_examples() {
    assert_eq!(header_alignment_changes(&[]), vec![]);
    assert_eq!(header_alignment_changes(&[4]), vec![4u32]);
}
