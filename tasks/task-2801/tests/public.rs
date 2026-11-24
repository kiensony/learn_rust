use task_2801::*;

#[test]
fn public_examples() {
    assert_eq!(service_acknowledgments_trace(&[]), vec![]);
    assert_eq!(service_acknowledgments_trace(&[4]), vec![4u32]);
}
