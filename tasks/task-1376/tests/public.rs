use task_1376::*;

#[test]
fn public_examples() {
    assert_eq!(frame_checksum_descending_steps(&[]), 0);
    assert_eq!(frame_checksum_descending_steps(&[0]), 0);
}
