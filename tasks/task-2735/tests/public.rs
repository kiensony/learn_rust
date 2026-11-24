use task_2735::*;

#[test]
fn public_examples() {
    assert_eq!(cli_rust_sources_changes(""), vec![]);
    assert_eq!(cli_rust_sources_changes("a.txt"), vec![]);
}
