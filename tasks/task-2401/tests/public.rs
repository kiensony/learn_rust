use task_2401::*;

#[test]
fn public_examples() {
    assert_eq!(header_alignment_trace(&[]), vec![]);
    assert_eq!(header_alignment_trace(&[4]), vec![4u32]);
}
