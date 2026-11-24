use task_2730::*;

#[test]
fn public_examples() {
    assert_eq!(cli_path_depth_reverse(""), vec![]);
    assert_eq!(cli_path_depth_reverse("a.txt"), vec![1u32]);
}
