use task_1741::*;

#[test]
fn public_examples() {
    assert_eq!(rotate_macro_trace(&[]), vec![]);
    assert_eq!(rotate_macro_trace(&[4]), vec![16u32]);
}
