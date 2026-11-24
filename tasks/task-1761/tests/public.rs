use task_1761::*;

#[test]
fn public_examples() {
    assert_eq!(generated_offset_trace(&[]), vec![]);
    assert_eq!(generated_offset_trace(&[4]), vec![11u32]);
}
