use task_2326::*;

#[test]
fn public_examples() {
    assert_eq!(batch_medians_descending_steps(&[]), 0);
    assert_eq!(batch_medians_descending_steps(&[4]), 0);
}
