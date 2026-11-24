use task_2715::*;

#[test]
fn public_examples() {
    assert_eq!(cli_extensions_changes(""), vec![]);
    assert_eq!(cli_extensions_changes("a.txt"), vec![3u32]);
}
