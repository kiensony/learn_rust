use task_2810::*;

#[test]
fn public_examples() {
    assert_eq!(service_acknowledgments_reverse(&[]), vec![]);
    assert_eq!(service_acknowledgments_reverse(&[4]), vec![4u32]);
}
