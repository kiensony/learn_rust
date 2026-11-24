use task_2423::*;

#[test]
fn public_examples() {
    assert_eq!(wide_alignment_bounds(&[]), None);
    assert_eq!(wide_alignment_bounds(&[4]), Some((8u32, 8u32)));
}
