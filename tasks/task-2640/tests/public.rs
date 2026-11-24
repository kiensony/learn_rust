use task_2640::*;

#[test]
fn public_examples() {
    assert_eq!(embedded_saturation_reverse(&[]), vec![]);
    assert_eq!(embedded_saturation_reverse(&[4]), vec![8u32]);
}
