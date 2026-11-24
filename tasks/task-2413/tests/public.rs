use task_2413::*;

#[test]
fn public_examples() {
    assert_eq!(word_alignment_bounds(&[]), None);
    assert_eq!(word_alignment_bounds(&[4]), Some((4u32, 4u32)));
}
