use task_2741::*;

#[test]
fn public_examples() {
    assert_eq!(cli_stems_trace(""), vec![]);
    assert_eq!(cli_stems_trace("a.txt"), vec![1u32]);
}
