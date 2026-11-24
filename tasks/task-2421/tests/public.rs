use task_2421::*;

#[test]
fn public_examples() {
    assert_eq!(wide_alignment_trace(&[]), vec![]);
    assert_eq!(wide_alignment_trace(&[4]), vec![8u32]);
}
