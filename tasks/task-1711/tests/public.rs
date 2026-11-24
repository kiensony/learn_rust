use task_1711::*;

#[test]
fn public_examples() {
    assert_eq!(saturating_macro_trace(&[]), vec![]);
    assert_eq!(saturating_macro_trace(&[4]), vec![8u32]);
}
