use task_2728::*;

#[test]
fn public_examples() {
    assert_eq!(cli_path_depth_steps(""), vec![]);
    assert_eq!(cli_path_depth_steps("a.txt"), vec![]);
}
