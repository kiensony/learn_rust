use task_1262::*;

#[test]
fn public_examples() {
    assert_eq!(positional_options_total(""), 0);
    assert_eq!(positional_options_total("only"), 4);
}
