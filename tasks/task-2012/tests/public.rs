use task_2012::*;

#[test]
fn public_examples() {
    assert_eq!(polled_flags_total(&[]), 0);
    assert_eq!(polled_flags_total(&[4]), 1);
}
