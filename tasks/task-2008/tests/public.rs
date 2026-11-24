use task_2008::*;

#[test]
fn public_examples() {
    assert_eq!(polled_lengths_steps(&[]), vec![]);
    assert_eq!(polled_lengths_steps(&[4]), vec![]);
}
