use task_2120::*;

#[test]
fn public_examples() {
    assert_eq!(priority_removal_reverse(&[]), vec![]);
    assert_eq!(priority_removal_reverse(&[4]), vec![4u32]);
}
