use task_2388::*;

#[test]
fn public_examples() {
    assert_eq!(rolling_parity_steps(&[]), vec![]);
    assert_eq!(rolling_parity_steps(&[4]), vec![]);
}
