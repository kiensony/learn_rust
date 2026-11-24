use task_1374::*;

#[test]
fn public_examples() {
    assert_eq!(frame_checksum_peaks(&[]), vec![]);
    assert_eq!(frame_checksum_peaks(&[0]), vec![0u32]);
}
