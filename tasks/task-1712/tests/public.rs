use task_1712::*;

#[test]
fn public_examples() {
    assert_eq!(saturating_macro_total(&[]), 0);
    assert_eq!(saturating_macro_total(&[4]), 8);
}
