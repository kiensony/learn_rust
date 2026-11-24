use task_1375::*;

#[test]
fn public_examples() {
    assert_eq!(frame_checksum_changes(&[]), vec![]);
    assert_eq!(frame_checksum_changes(&[0]), vec![0u32]);
}
