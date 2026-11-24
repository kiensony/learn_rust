use task_2330::*;

#[test]
fn public_examples() {
    assert_eq!(batch_medians_reverse(&[]), vec![]);
    assert_eq!(batch_medians_reverse(&[4]), vec![4u32]);
}
