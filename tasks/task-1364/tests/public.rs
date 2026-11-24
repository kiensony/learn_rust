use task_1364::*;

#[test]
fn public_examples() {
    assert_eq!(frame_offsets_peaks(&[]), vec![]);
    assert_eq!(frame_offsets_peaks(&[0]), vec![0u32]);
}
