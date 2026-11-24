use task_1265::*;

#[test]
fn public_examples() {
    assert_eq!(positional_options_changes(""), vec![]);
    assert_eq!(positional_options_changes("only"), vec![4u32]);
}
