use task_2738::*;

#[test]
fn public_examples() {
    assert_eq!(cli_rust_sources_steps(""), vec![]);
    assert_eq!(cli_rust_sources_steps("a.txt"), vec![]);
}
