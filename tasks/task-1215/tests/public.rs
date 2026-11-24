use task_1215::*;

#[test]
fn public_examples() {
    assert_eq!(read_chunks_changes(&[]), vec![]);
    assert_eq!(read_chunks_changes(&[4]), vec![1u32]);
}
