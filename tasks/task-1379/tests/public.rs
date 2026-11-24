use task_1379::*;

#[test]
fn public_examples() {
    assert_eq!(frame_checksum_longest_run(&[]), 0);
    assert_eq!(frame_checksum_longest_run(&[0]), 1);
}
