use task_2711::*;

#[test]
fn public_examples() {
    assert_eq!(cli_extensions_trace(""), vec![]);
    assert_eq!(cli_extensions_trace("a.txt"), vec![3u32]);
}
