use task_2722::*;

#[test]
fn public_examples() {
    assert_eq!(cli_path_depth_total(""), 0);
    assert_eq!(cli_path_depth_total("a.txt"), 1);
}
