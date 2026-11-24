use task_2617::*;

#[test]
fn public_examples() {
    assert_eq!(embedded_flags_distinct(&[]), vec![]);
    assert_eq!(embedded_flags_distinct(&[4]), vec![4u32]);
}
