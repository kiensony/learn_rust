use task_2402::*;

#[test]
fn public_examples() {
    assert_eq!(header_alignment_total(&[]), 0);
    assert_eq!(header_alignment_total(&[4]), 4);
}
