use task_1218::*;

#[test]
fn public_examples() {
    assert_eq!(read_chunks_steps(&[]), vec![]);
    assert_eq!(read_chunks_steps(&[4]), vec![]);
}
