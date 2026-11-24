use task_2382::*;

#[test]
fn public_examples() {
    assert_eq!(rolling_parity_total(&[]), 0);
    assert_eq!(rolling_parity_total(&[4]), 4);
}
