use task_2336::*;

#[test]
fn public_examples() {
    assert_eq!(batch_variation_descending_steps(&[]), 0);
    assert_eq!(batch_variation_descending_steps(&[4]), 0);
}
