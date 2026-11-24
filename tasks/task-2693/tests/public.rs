use task_2693::*;

#[test]
fn public_examples() {
    assert_eq!(universal_slice_width_bounds(&[]), None);
    assert_eq!(universal_slice_width_bounds(&[4]), Some((0u32, 0u32)));
}
