use task_2169::*;

#[test]
fn public_examples() {
    assert_eq!(sorted_insertion_moves_longest_run(&[]), 0);
    assert_eq!(sorted_insertion_moves_longest_run(&[4]), 1);
}
