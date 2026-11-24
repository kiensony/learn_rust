use task_2117::*;

#[test]
fn public_examples() {
    assert_eq!(priority_removal_distinct(&[]), vec![]);
    assert_eq!(priority_removal_distinct(&[4]), vec![4u32]);
}
