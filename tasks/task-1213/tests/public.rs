use task_1213::*;

#[test]
fn public_examples() {
    assert_eq!(read_chunks_bounds(&[]), None);
    assert_eq!(read_chunks_bounds(&[4]), Some((1u32, 1u32)));
}
