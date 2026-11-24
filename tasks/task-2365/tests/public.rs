use task_2365::*;

#[test]
fn public_examples() {
    assert_eq!(single_pass_prefix_minimum_changes(&[]), vec![]);
    assert_eq!(single_pass_prefix_minimum_changes(&[4]), vec![4u32]);
}
