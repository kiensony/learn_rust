use task_2325::*;

#[test]
fn public_examples() {
    assert_eq!(batch_medians_changes(&[]), vec![]);
    assert_eq!(batch_medians_changes(&[4]), vec![4u32]);
}
