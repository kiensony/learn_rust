use task_2695::*;

#[test]
fn public_examples() {
    assert_eq!(universal_slice_width_changes(&[]), vec![]);
    assert_eq!(universal_slice_width_changes(&[4]), vec![0u32]);
}
