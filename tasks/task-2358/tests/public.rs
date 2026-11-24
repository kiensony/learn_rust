use task_2358::*;

#[test]
fn public_examples() {
    assert_eq!(single_pass_prefix_maximum_steps(&[]), vec![]);
    assert_eq!(single_pass_prefix_maximum_steps(&[4]), vec![]);
}
