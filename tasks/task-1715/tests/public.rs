use task_1715::*;

#[test]
fn public_examples() {
    assert_eq!(saturating_macro_changes(&[]), vec![]);
    assert_eq!(saturating_macro_changes(&[4]), vec![8u32]);
}
