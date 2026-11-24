use task_2807::*;

#[test]
fn public_examples() {
    assert_eq!(service_acknowledgments_distinct(&[]), vec![]);
    assert_eq!(service_acknowledgments_distinct(&[4]), vec![4u32]);
}
