use task_1772::*;

#[test]
fn public_examples() {
    assert_eq!(build_mask_total(&[]), 0);
    assert_eq!(build_mask_total(&[4]), 4);
}
