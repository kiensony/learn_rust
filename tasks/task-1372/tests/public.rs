use task_1372::*;

#[test]
fn public_examples() {
    assert_eq!(frame_checksum_total(&[]), 0);
    assert_eq!(frame_checksum_total(&[0]), 0);
}
