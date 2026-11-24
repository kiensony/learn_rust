use task_2724::*;

#[test]
fn public_examples() {
    assert_eq!(cli_path_depth_peaks(""), vec![]);
    assert_eq!(cli_path_depth_peaks("a.txt"), vec![1u32]);
}
