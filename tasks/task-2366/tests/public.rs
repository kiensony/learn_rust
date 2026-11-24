use task_2366::*;

#[test]
fn public_examples() {
    assert_eq!(single_pass_prefix_minimum_descending_steps(&[]), 0);
    assert_eq!(single_pass_prefix_minimum_descending_steps(&[4]), 0);
}
