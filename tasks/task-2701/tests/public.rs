use task_2701::*;

#[test]
fn public_examples() {
    assert_eq!(cli_basenames_trace(""), vec![]);
    assert_eq!(cli_basenames_trace("a.txt"), vec![5u32]);
}
