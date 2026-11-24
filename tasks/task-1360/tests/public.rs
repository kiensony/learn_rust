use task_1360::*;

#[test]
fn public_examples() {
    assert_eq!(frame_lengths_reverse(&[]), vec![]);
    assert_eq!(frame_lengths_reverse(&[0]), vec![0u32]);
}
