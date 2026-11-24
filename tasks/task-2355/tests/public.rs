use task_2355::*;

#[test]
fn public_examples() {
    assert_eq!(single_pass_prefix_maximum_changes(&[]), vec![]);
    assert_eq!(single_pass_prefix_maximum_changes(&[4]), vec![4u32]);
}
