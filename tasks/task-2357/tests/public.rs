use task_2357::*;

#[test]
fn public_examples() {
    assert_eq!(single_pass_prefix_maximum_distinct(&[]), vec![]);
    assert_eq!(single_pass_prefix_maximum_distinct(&[4]), vec![4u32]);
}
