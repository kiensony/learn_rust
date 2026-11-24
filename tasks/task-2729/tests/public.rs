use task_2729::*;

#[test]
fn public_examples() {
    assert_eq!(cli_path_depth_longest_run(""), 0);
    assert_eq!(cli_path_depth_longest_run("a.txt"), 1);
}
