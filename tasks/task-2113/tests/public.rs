use task_2113::*;

#[test]
fn public_examples() {
    assert_eq!(priority_removal_bounds(&[]), None);
    assert_eq!(priority_removal_bounds(&[4]), Some((4u32, 4u32)));
}
