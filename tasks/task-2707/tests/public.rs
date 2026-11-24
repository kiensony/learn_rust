use task_2707::*;

#[test]
fn public_examples() {
    assert_eq!(cli_basenames_distinct(""), vec![]);
    assert_eq!(cli_basenames_distinct("a.txt"), vec![5u32]);
}
