use task_2363::*;

#[test]
fn public_examples() {
    assert_eq!(single_pass_prefix_minimum_bounds(&[]), None);
    assert_eq!(single_pass_prefix_minimum_bounds(&[4]), Some((4u32, 4u32)));
}
