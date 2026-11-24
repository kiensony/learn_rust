use task_2367::*;

#[test]
fn public_examples() {
    assert_eq!(single_pass_prefix_minimum_distinct(&[]), vec![]);
    assert_eq!(single_pass_prefix_minimum_distinct(&[4]), vec![4u32]);
}
