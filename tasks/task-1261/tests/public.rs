use task_1261::*;

#[test]
fn public_examples() {
    assert_eq!(positional_options_trace(""), vec![]);
    assert_eq!(positional_options_trace("only"), vec![4u32]);
}
