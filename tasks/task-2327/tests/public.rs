use task_2327::*;

#[test]
fn public_examples() {
    assert_eq!(batch_medians_distinct(&[]), vec![]);
    assert_eq!(batch_medians_distinct(&[4]), vec![4u32]);
}
