use task_1716::*;

#[test]
fn public_examples() {
    assert_eq!(saturating_macro_descending_steps(&[]), 0);
    assert_eq!(saturating_macro_descending_steps(&[4]), 0);
}
