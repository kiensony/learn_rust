use task_2408::*;

#[test]
fn public_examples() {
    assert_eq!(header_alignment_steps(&[]), vec![]);
    assert_eq!(header_alignment_steps(&[4]), vec![]);
}
