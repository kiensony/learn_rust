use task_2718::*;

#[test]
fn public_examples() {
    assert_eq!(cli_extensions_steps(""), vec![]);
    assert_eq!(cli_extensions_steps("a.txt"), vec![]);
}
