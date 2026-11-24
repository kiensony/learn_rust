use task_2745::*;

#[test]
fn public_examples() {
    assert_eq!(cli_stems_changes(""), vec![]);
    assert_eq!(cli_stems_changes("a.txt"), vec![1u32]);
}
