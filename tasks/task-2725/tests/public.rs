use task_2725::*;

#[test]
fn public_examples() {
    assert_eq!(cli_path_depth_changes(""), vec![]);
    assert_eq!(cli_path_depth_changes("a.txt"), vec![1u32]);
}
