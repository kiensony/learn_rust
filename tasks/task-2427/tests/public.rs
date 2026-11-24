use task_2427::*;

#[test]
fn public_examples() {
    assert_eq!(wide_alignment_distinct(&[]), vec![]);
    assert_eq!(wide_alignment_distinct(&[4]), vec![8u32]);
}
