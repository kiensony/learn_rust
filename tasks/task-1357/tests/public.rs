use task_1357::*;

#[test]
fn public_examples() {
    assert_eq!(frame_lengths_distinct(&[]), vec![]);
    assert_eq!(frame_lengths_distinct(&[0]), vec![0u32]);
}
