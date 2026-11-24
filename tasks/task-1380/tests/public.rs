use task_1380::*;

#[test]
fn public_examples() {
    assert_eq!(frame_checksum_reverse(&[]), vec![]);
    assert_eq!(frame_checksum_reverse(&[0]), vec![0u32]);
}
