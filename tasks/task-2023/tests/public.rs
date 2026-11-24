use task_2023::*;

#[test]
fn public_examples() {
    assert_eq!(polled_groups_bounds(&[]), None);
    assert_eq!(polled_groups_bounds(&[4]), Some((1u32, 1u32)));
}
