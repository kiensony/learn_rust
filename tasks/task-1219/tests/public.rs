use task_1219::*;

#[test]
fn public_examples() {
    assert_eq!(read_chunks_longest_run(&[]), 0);
    assert_eq!(read_chunks_longest_run(&[4]), 1);
}
