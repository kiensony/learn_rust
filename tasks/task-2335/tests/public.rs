use task_2335::*;

#[test]
fn public_examples() {
    assert_eq!(batch_variation_changes(&[]), vec![]);
    assert_eq!(batch_variation_changes(&[4]), vec![0u32]);
}
