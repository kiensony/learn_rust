use task_1717::*;

#[test]
fn public_examples() {
    assert_eq!(saturating_macro_distinct(&[]), vec![]);
    assert_eq!(saturating_macro_distinct(&[4]), vec![8u32]);
}
