use task_1384::*;

#[test]
fn public_examples() {
    assert_eq!(first_frame_byte_peaks(&[]), vec![]);
    assert_eq!(first_frame_byte_peaks(&[0]), vec![0u32]);
}
