use task_2744::*;

#[test]
fn public_examples() {
    assert_eq!(cli_stems_peaks(""), vec![]);
    assert_eq!(cli_stems_peaks("a.txt"), vec![1u32]);
}
