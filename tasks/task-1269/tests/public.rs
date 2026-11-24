use task_1269::*;

#[test]
fn public_examples() {
    assert_eq!(positional_options_longest_run(""), 0);
    assert_eq!(positional_options_longest_run("only"), 1);
}
