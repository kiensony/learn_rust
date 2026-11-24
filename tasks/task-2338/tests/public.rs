use task_2338::*;

#[test]
fn public_examples() {
    assert_eq!(batch_variation_steps(&[]), vec![]);
    assert_eq!(batch_variation_steps(&[4]), vec![]);
}
