use task_2321::*;

#[test]
fn public_examples() {
    assert_eq!(batch_medians_trace(&[]), vec![]);
    assert_eq!(batch_medians_trace(&[4]), vec![4u32]);
}
