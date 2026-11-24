use task_1268::*;

#[test]
fn public_examples() {
    assert_eq!(positional_options_steps(""), vec![]);
    assert_eq!(positional_options_steps("only"), vec![]);
}
