use task_1767::*;

#[test]
fn public_examples() {
    assert_eq!(generated_offset_distinct(&[]), vec![]);
    assert_eq!(generated_offset_distinct(&[4]), vec![11u32]);
}
