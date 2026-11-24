use task_2709::*;

#[test]
fn public_examples() {
    assert_eq!(cli_basenames_longest_run(""), 0);
    assert_eq!(cli_basenames_longest_run("a.txt"), 1);
}
