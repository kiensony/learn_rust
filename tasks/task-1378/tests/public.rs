use task_1378::*;

#[test]
fn public_examples() {
    assert_eq!(frame_checksum_steps(&[]), vec![]);
    assert_eq!(frame_checksum_steps(&[0]), vec![]);
}
