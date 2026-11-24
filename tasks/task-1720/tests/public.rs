use task_1720::*;

#[test]
fn public_examples() {
    assert_eq!(saturating_macro_reverse(&[]), vec![]);
    assert_eq!(saturating_macro_reverse(&[4]), vec![8u32]);
}
