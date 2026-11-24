use task_2368::*;

#[test]
fn public_examples() {
    assert_eq!(single_pass_prefix_minimum_steps(&[]), vec![]);
    assert_eq!(single_pass_prefix_minimum_steps(&[4]), vec![]);
}
