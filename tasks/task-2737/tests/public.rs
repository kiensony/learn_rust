use task_2737::*;

#[test]
fn public_examples() {
    assert_eq!(cli_rust_sources_distinct(""), vec![]);
    assert_eq!(cli_rust_sources_distinct("a.txt"), vec![]);
}
