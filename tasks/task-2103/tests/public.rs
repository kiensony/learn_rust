use task_2103::*;

#[test]
fn public_examples() {
    assert_eq!(sorted_keys_bounds(&[]), None);
    assert_eq!(sorted_keys_bounds(&[4]), Some((4u32, 4u32)));
}
