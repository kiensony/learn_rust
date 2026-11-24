use task_1778::*;

#[test]
fn public_examples() {
    assert_eq!(build_mask_steps(&[]), vec![]);
    assert_eq!(build_mask_steps(&[4]), vec![]);
}
