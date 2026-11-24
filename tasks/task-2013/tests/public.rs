use task_2013::*;

#[test]
fn public_examples() {
    assert_eq!(polled_flags_bounds(&[]), None);
    assert_eq!(polled_flags_bounds(&[4]), Some((1u32, 1u32)));
}
