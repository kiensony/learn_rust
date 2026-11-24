use task_1367::*;

#[test]
fn public_examples() {
    assert_eq!(frame_offsets_distinct(&[]), vec![]);
    assert_eq!(frame_offsets_distinct(&[0]), vec![0u32]);
}
