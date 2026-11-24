use task_1394::*;

#[test]
fn public_examples() {
    assert_eq!(frame_maxima_peaks(&[]), vec![]);
    assert_eq!(frame_maxima_peaks(&[0]), vec![0u32]);
}
