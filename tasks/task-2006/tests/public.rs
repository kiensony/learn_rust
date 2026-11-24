use task_2006::*;

#[test]
fn public_examples() {
    assert_eq!(polled_lengths_descending_steps(&[]), 0);
    assert_eq!(polled_lengths_descending_steps(&[4]), 0);
}
