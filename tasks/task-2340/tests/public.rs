use task_2340::*;

#[test]
fn public_examples() {
    assert_eq!(batch_variation_reverse(&[]), vec![]);
    assert_eq!(batch_variation_reverse(&[4]), vec![0u32]);
}
