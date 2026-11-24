use task_1352::*;

#[test]
fn public_examples() {
    assert_eq!(frame_lengths_total(&[]), 0);
    assert_eq!(frame_lengths_total(&[0]), 0);
}
