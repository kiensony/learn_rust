use task_2717::*;

#[test]
fn public_examples() {
    assert_eq!(cli_extensions_distinct(""), vec![]);
    assert_eq!(cli_extensions_distinct("a.txt"), vec![3u32]);
}
