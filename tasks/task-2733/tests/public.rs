use task_2733::*;

#[test]
fn public_examples() {
    assert_eq!(cli_rust_sources_bounds(""), None);
    assert_eq!(cli_rust_sources_bounds("a.txt"), None);
}
