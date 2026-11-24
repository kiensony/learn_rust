use task_1371::*;

#[test]
fn public_examples() {
    assert_eq!(frame_checksum_trace(&[]), vec![]);
    assert_eq!(frame_checksum_trace(&[0]), vec![0u32]);
}
