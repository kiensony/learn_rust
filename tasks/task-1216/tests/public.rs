use task_1216::*;

#[test]
fn public_examples() {
    assert_eq!(read_chunks_descending_steps(&[]), 0);
    assert_eq!(read_chunks_descending_steps(&[4]), 0);
}
