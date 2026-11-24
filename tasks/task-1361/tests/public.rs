use task_1361::*;

#[test]
fn public_examples() {
    assert_eq!(frame_offsets_trace(&[]), vec![]);
    assert_eq!(frame_offsets_trace(&[0]), vec![0u32]);
}
