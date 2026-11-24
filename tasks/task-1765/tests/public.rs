use task_1765::*;

#[test]
fn public_examples() {
    assert_eq!(generated_offset_changes(&[]), vec![]);
    assert_eq!(generated_offset_changes(&[4]), vec![11u32]);
}
