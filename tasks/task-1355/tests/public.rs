use task_1355::*;

#[test]
fn public_examples() {
    assert_eq!(frame_lengths_changes(&[]), vec![]);
    assert_eq!(frame_lengths_changes(&[0]), vec![0u32]);
}
