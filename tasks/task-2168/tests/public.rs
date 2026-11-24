use task_2168::*;

#[test]
fn public_examples() {
    assert_eq!(sorted_insertion_moves_steps(&[]), vec![]);
    assert_eq!(sorted_insertion_moves_steps(&[4]), vec![]);
}
