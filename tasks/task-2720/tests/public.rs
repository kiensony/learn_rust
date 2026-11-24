use task_2720::*;

#[test]
fn public_examples() {
    assert_eq!(cli_extensions_reverse(""), vec![]);
    assert_eq!(cli_extensions_reverse("a.txt"), vec![3u32]);
}
