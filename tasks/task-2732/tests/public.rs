use task_2732::*;

#[test]
fn public_examples() {
    assert_eq!(cli_rust_sources_total(""), 0);
    assert_eq!(cli_rust_sources_total("a.txt"), 0);
}
