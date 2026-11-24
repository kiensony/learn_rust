use task_1713::*;

#[test]
fn public_examples() {
    assert_eq!(saturating_macro_bounds(&[]), None);
    assert_eq!(saturating_macro_bounds(&[4]), Some((8u32, 8u32)));
}
