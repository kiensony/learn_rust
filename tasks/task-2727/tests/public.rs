use task_2727::*;

#[test]
fn public_examples() {
    assert_eq!(cli_path_depth_distinct(""), vec![]);
    assert_eq!(cli_path_depth_distinct("a.txt"), vec![1u32]);
}
