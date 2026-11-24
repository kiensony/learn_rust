use task_2337::*;

#[test]
fn public_examples() {
    assert_eq!(batch_variation_distinct(&[]), vec![]);
    assert_eq!(batch_variation_distinct(&[4]), vec![0u32]);
}
