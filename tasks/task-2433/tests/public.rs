use task_2433::*;

#[test]
fn public_examples() {
    assert_eq!(block_alignment_bounds(&[]), None);
    assert_eq!(block_alignment_bounds(&[4]), Some((16u32, 16u32)));
}
