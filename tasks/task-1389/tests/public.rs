use task_1389::*;

#[test]
fn public_examples() {
    assert_eq!(first_frame_byte_longest_run(&[]), 0);
    assert_eq!(first_frame_byte_longest_run(&[0]), 1);
}
