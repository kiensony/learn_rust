use task_1369::*;

#[test]
fn public_examples() {
    assert_eq!(frame_offsets_longest_run(&[]), 0);
    assert_eq!(frame_offsets_longest_run(&[0]), 1);
}
