use task_2002::*;

#[test]
fn public_examples() {
    assert_eq!(polled_lengths_total(&[]), 0);
    assert_eq!(polled_lengths_total(&[4]), 4);
}
