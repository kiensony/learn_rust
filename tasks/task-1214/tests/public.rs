use task_1214::*;

#[test]
fn public_examples() {
    assert_eq!(read_chunks_peaks(&[]), vec![]);
    assert_eq!(read_chunks_peaks(&[4]), vec![1u32]);
}
