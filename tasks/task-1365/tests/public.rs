use task_1365::*;

#[test]
fn public_examples() {
    assert_eq!(frame_offsets_changes(&[]), vec![]);
    assert_eq!(frame_offsets_changes(&[0]), vec![0u32]);
}
