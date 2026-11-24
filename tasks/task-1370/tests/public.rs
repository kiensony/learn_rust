use task_1370::*;

#[test]
fn public_examples() {
    assert_eq!(frame_offsets_reverse(&[]), vec![]);
    assert_eq!(frame_offsets_reverse(&[0]), vec![0u32]);
}
