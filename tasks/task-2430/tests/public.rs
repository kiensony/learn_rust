use task_2430::*;

#[test]
fn public_examples() {
    assert_eq!(wide_alignment_reverse(&[]), vec![]);
    assert_eq!(wide_alignment_reverse(&[4]), vec![8u32]);
}
