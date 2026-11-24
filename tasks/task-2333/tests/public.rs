use task_2333::*;

#[test]
fn public_examples() {
    assert_eq!(batch_variation_bounds(&[]), None);
    assert_eq!(batch_variation_bounds(&[4]), Some((0u32, 0u32)));
}
