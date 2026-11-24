use task_2322::*;

#[test]
fn public_examples() {
    assert_eq!(batch_medians_total(&[]), 0);
    assert_eq!(batch_medians_total(&[4]), 4);
}
