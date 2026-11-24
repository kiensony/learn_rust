use task_2642::*;

#[test]
fn public_examples() {
    assert_eq!(embedded_nibble_swap_total(&[]), 0);
    assert_eq!(embedded_nibble_swap_total(&[4]), 64);
}
