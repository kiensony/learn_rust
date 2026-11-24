use task_2731::*;

#[test]
fn public_examples() {
    assert_eq!(cli_rust_sources_trace(""), vec![]);
    assert_eq!(cli_rust_sources_trace("a.txt"), vec![]);
}
