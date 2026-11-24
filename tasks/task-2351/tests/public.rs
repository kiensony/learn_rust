use task_2351::*;

#[test]
fn public_examples() {
    assert_eq!(single_pass_prefix_maximum_trace(&[]), vec![]);
    assert_eq!(single_pass_prefix_maximum_trace(&[4]), vec![4u32]);
}
