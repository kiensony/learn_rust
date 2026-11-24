use task_2425::*;

#[test]
fn public_examples() {
    assert_eq!(wide_alignment_changes(&[]), vec![]);
    assert_eq!(wide_alignment_changes(&[4]), vec![8u32]);
}
