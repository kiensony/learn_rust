use task_1263::*;

#[test]
fn public_examples() {
    assert_eq!(positional_options_bounds(""), None);
    assert_eq!(positional_options_bounds("only"), Some((4u32, 4u32)));
}
