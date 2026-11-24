use task_2360::*;

#[test]
fn public_examples() {
    assert_eq!(single_pass_prefix_maximum_reverse(&[]), vec![]);
    assert_eq!(single_pass_prefix_maximum_reverse(&[4]), vec![4u32]);
}
