use task_1236::*;

#[test]
fn public_examples() {
    assert_eq!(seek_offsets_descending_steps(&[]), 0);
    assert_eq!(seek_offsets_descending_steps(&[4]), 0);
}
