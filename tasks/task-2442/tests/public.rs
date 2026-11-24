use task_2442::*;

#[test]
fn public_examples() {
    assert_eq!(page_alignment_total(&[]), 0);
    assert_eq!(page_alignment_total(&[4]), 32);
}
