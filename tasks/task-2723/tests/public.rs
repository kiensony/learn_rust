use task_2723::*;

#[test]
fn public_examples() {
    assert_eq!(cli_path_depth_bounds(""), None);
    assert_eq!(cli_path_depth_bounds("a.txt"), Some((1u32, 1u32)));
}
