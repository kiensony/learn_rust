use task_2713::*;

#[test]
fn public_examples() {
    assert_eq!(cli_extensions_bounds(""), None);
    assert_eq!(cli_extensions_bounds("a.txt"), Some((3u32, 3u32)));
}
