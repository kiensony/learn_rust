use task_2726::*;

#[test]
fn public_examples() {
    assert_eq!(cli_path_depth_descending_steps(""), 0);
    assert_eq!(cli_path_depth_descending_steps("a.txt"), 0);
}
