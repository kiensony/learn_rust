use task_2716::*;

#[test]
fn public_examples() {
    assert_eq!(cli_extensions_descending_steps(""), 0);
    assert_eq!(cli_extensions_descending_steps("a.txt"), 0);
}
