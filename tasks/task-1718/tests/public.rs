use task_1718::*;

#[test]
fn public_examples() {
    assert_eq!(saturating_macro_steps(&[]), vec![]);
    assert_eq!(saturating_macro_steps(&[4]), vec![]);
}
