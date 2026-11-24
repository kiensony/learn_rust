use task_2323::*;

#[test]
fn public_examples() {
    assert_eq!(batch_medians_bounds(&[]), None);
    assert_eq!(batch_medians_bounds(&[4]), Some((4u32, 4u32)));
}
