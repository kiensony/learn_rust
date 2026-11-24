use task_1217::*;

#[test]
fn public_examples() {
    assert_eq!(read_chunks_distinct(&[]), vec![]);
    assert_eq!(read_chunks_distinct(&[4]), vec![1u32]);
}
