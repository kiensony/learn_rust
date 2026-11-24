use task_1377::*;

#[test]
fn public_examples() {
    assert_eq!(frame_checksum_distinct(&[]), vec![]);
    assert_eq!(frame_checksum_distinct(&[0]), vec![0u32]);
}
