use task_1773::*;

#[test]
fn public_examples() {
    assert_eq!(build_mask_bounds(&[]), None);
    assert_eq!(build_mask_bounds(&[4]), Some((4u32, 4u32)));
}
