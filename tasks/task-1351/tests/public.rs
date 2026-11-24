use task_1351::*;

#[test]
fn public_examples() {
    assert_eq!(frame_lengths_trace(&[]), vec![]);
    assert_eq!(frame_lengths_trace(&[0]), vec![0u32]);
}
