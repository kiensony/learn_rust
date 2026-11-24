use task_2332::*;

#[test]
fn public_examples() {
    assert_eq!(batch_variation_total(&[]), 0);
    assert_eq!(batch_variation_total(&[4]), 0);
}
