use task_2714::*;

#[test]
fn public_examples() {
    assert_eq!(cli_extensions_peaks(""), vec![]);
    assert_eq!(cli_extensions_peaks("a.txt"), vec![3u32]);
}
