use task_1723::*;

#[test]
fn public_examples() {
    assert_eq!(boolean_macro_bounds(&[]), None);
    assert_eq!(boolean_macro_bounds(&[4]), Some((0u32, 0u32)));
}
