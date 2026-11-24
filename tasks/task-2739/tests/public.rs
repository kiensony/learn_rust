use task_2739::*;

#[test]
fn public_examples() {
    assert_eq!(cli_rust_sources_longest_run(""), 0);
    assert_eq!(cli_rust_sources_longest_run("a.txt"), 0);
}
