use task_2328::*;

#[test]
fn public_examples() {
    assert_eq!(batch_medians_steps(&[]), vec![]);
    assert_eq!(batch_medians_steps(&[4]), vec![]);
}
