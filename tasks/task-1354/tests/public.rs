use task_1354::*;

#[test]
fn public_examples() {
    assert_eq!(frame_lengths_peaks(&[]), vec![]);
    assert_eq!(frame_lengths_peaks(&[0]), vec![0u32]);
}
