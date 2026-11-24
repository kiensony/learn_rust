use task_2353::*;

#[test]
fn public_examples() {
    assert_eq!(single_pass_prefix_maximum_bounds(&[]), None);
    assert_eq!(single_pass_prefix_maximum_bounds(&[4]), Some((4u32, 4u32)));
}
