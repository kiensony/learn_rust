use task_1973::*;

#[test]
fn public_examples() {
    assert_eq!(atomic_toggle_bounds(&[]), None);
    assert_eq!(atomic_toggle_bounds(&[4]), Some((0u32, 0u32)));
}
