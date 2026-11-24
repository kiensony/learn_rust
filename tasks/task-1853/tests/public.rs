use task_1853::*;

#[test]
fn public_examples() {
    assert_eq!(shared_read_handles_bounds(&[]), None);
    assert_eq!(shared_read_handles_bounds(&[4]), Some((1u32, 1u32)));
}
