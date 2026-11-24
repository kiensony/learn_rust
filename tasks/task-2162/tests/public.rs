use task_2162::*;

#[test]
fn public_examples() {
    assert_eq!(sorted_insertion_moves_total(&[]), 0);
    assert_eq!(sorted_insertion_moves_total(&[4]), 0);
}
