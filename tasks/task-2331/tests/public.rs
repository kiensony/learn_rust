use task_2331::*;

#[test]
fn public_examples() {
    assert_eq!(batch_variation_trace(&[]), vec![]);
    assert_eq!(batch_variation_trace(&[4]), vec![0u32]);
}
