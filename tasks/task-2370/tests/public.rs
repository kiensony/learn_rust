use task_2370::*;

#[test]
fn public_examples() {
    assert_eq!(single_pass_prefix_minimum_reverse(&[]), vec![]);
    assert_eq!(single_pass_prefix_minimum_reverse(&[4]), vec![4u32]);
}
