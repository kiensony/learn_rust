use task_1771::*;

#[test]
fn public_examples() {
    assert_eq!(build_mask_trace(&[]), vec![]);
    assert_eq!(build_mask_trace(&[4]), vec![4u32]);
}
