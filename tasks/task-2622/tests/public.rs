use task_2622::*;

#[test]
fn public_examples() {
    assert_eq!(embedded_parity_total(&[]), 0);
    assert_eq!(embedded_parity_total(&[4]), 1);
}
