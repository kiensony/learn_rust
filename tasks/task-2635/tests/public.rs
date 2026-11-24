use task_2635::*;

#[test]
fn public_examples() {
    assert_eq!(embedded_saturation_changes(&[]), vec![]);
    assert_eq!(embedded_saturation_changes(&[4]), vec![8u32]);
}
