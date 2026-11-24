use task_1212::*;

#[test]
fn public_examples() {
    assert_eq!(read_chunks_total(&[]), 0);
    assert_eq!(read_chunks_total(&[4]), 1);
}
