use task_2721::*;

#[test]
fn public_examples() {
    assert_eq!(cli_path_depth_trace(""), vec![]);
    assert_eq!(cli_path_depth_trace("a.txt"), vec![1u32]);
}
