use task_2165::*;

#[test]
fn public_examples() {
    assert_eq!(sorted_insertion_moves_changes(&[]), vec![]);
    assert_eq!(sorted_insertion_moves_changes(&[4]), vec![0u32]);
}
