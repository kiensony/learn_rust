use task_1220::*;

#[test]
fn public_examples() {
    assert_eq!(read_chunks_reverse(&[]), vec![]);
    assert_eq!(read_chunks_reverse(&[4]), vec![1u32]);
}
