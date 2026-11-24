use task_2170::*;

#[test]
fn public_examples() {
    assert_eq!(sorted_insertion_moves_reverse(&[]), vec![]);
    assert_eq!(sorted_insertion_moves_reverse(&[4]), vec![0u32]);
}
