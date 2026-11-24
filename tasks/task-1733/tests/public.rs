use task_1733::*;

#[test]
fn public_examples() {
    assert_eq!(population_macro_bounds(&[]), None);
    assert_eq!(population_macro_bounds(&[4]), Some((1u32, 1u32)));
}
