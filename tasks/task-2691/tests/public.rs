use task_2691::*;

#[test]
fn public_examples() {
    assert_eq!(universal_slice_width_trace(&[]), vec![]);
    assert_eq!(universal_slice_width_trace(&[4]), vec![0u32]);
}
