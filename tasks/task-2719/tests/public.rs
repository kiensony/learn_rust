use task_2719::*;

#[test]
fn public_examples() {
    assert_eq!(cli_extensions_longest_run(""), 0);
    assert_eq!(cli_extensions_longest_run("a.txt"), 1);
}
