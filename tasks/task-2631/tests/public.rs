use task_2631::*;

#[test]
fn public_examples() {
    assert_eq!(embedded_saturation_trace(&[]), vec![]);
    assert_eq!(embedded_saturation_trace(&[4]), vec![8u32]);
}
