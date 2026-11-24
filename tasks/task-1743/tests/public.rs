use task_1743::*;

#[test]
fn public_examples() {
    assert_eq!(rotate_macro_bounds(&[]), None);
    assert_eq!(rotate_macro_bounds(&[4]), Some((16u32, 16u32)));
}
