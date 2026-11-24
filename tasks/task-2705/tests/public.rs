use task_2705::*;

#[test]
fn public_examples() {
    assert_eq!(cli_basenames_changes(""), vec![]);
    assert_eq!(cli_basenames_changes("a.txt"), vec![5u32]);
}
