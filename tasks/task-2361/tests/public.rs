use task_2361::*;

#[test]
fn public_examples() {
    assert_eq!(single_pass_prefix_minimum_trace(&[]), vec![]);
    assert_eq!(single_pass_prefix_minimum_trace(&[4]), vec![4u32]);
}
