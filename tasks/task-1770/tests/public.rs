use task_1770::*;

#[test]
fn public_examples() {
    assert_eq!(generated_offset_reverse(&[]), vec![]);
    assert_eq!(generated_offset_reverse(&[4]), vec![11u32]);
}
