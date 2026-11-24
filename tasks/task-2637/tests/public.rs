use task_2637::*;

#[test]
fn public_examples() {
    assert_eq!(embedded_saturation_distinct(&[]), vec![]);
    assert_eq!(embedded_saturation_distinct(&[4]), vec![8u32]);
}
