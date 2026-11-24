use task_2736::*;

#[test]
fn public_examples() {
    assert_eq!(cli_rust_sources_descending_steps(""), 0);
    assert_eq!(cli_rust_sources_descending_steps("a.txt"), 0);
}
