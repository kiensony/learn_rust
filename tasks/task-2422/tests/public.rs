use task_2422::*;

#[test]
fn public_examples() {
    assert_eq!(wide_alignment_total(&[]), 0);
    assert_eq!(wide_alignment_total(&[4]), 8);
}
