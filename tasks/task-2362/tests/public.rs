use task_2362::*;

#[test]
fn public_examples() {
    assert_eq!(single_pass_prefix_minimum_total(&[]), 0);
    assert_eq!(single_pass_prefix_minimum_total(&[4]), 4);
}
