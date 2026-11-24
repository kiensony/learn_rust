use task_2042::*;

#[test]
fn public_examples() {
    assert_eq!(polled_costs_total(&[]), 0);
    assert_eq!(polled_costs_total(&[4]), 1);
}
