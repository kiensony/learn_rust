use task_2445::*;

#[test]
fn public_examples() {
    assert_eq!(page_alignment_changes(&[]), vec![]);
    assert_eq!(page_alignment_changes(&[4]), vec![32u32]);
}
