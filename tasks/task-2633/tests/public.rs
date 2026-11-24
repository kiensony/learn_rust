use task_2633::*;

#[test]
fn public_examples() {
    assert_eq!(embedded_saturation_bounds(&[]), None);
    assert_eq!(embedded_saturation_bounds(&[4]), Some((8u32, 8u32)));
}
