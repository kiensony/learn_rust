use task_2386::*;

#[test]
fn public_examples() {
    assert_eq!(rolling_parity_descending_steps(&[]), 0);
    assert_eq!(rolling_parity_descending_steps(&[4]), 0);
}
