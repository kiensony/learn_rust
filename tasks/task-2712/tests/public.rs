use task_2712::*;

#[test]
fn public_examples() {
    assert_eq!(cli_extensions_total(""), 0);
    assert_eq!(cli_extensions_total("a.txt"), 3);
}
