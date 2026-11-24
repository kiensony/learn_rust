use task_2167::*;

#[test]
fn public_examples() {
    assert_eq!(sorted_insertion_moves_distinct(&[]), vec![]);
    assert_eq!(sorted_insertion_moves_distinct(&[4]), vec![0u32]);
}
