use task_1211::*;

#[test]
fn public_examples() {
    assert_eq!(read_chunks_trace(&[]), vec![]);
    assert_eq!(read_chunks_trace(&[4]), vec![1u32]);
}
