use task_1863::*;

#[test]
fn public_examples() {
    assert_eq!(weak_read_handles_bounds(&[]), None);
    assert_eq!(weak_read_handles_bounds(&[4]), Some((0u32, 0u32)));
}
