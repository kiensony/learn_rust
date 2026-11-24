use task_2742::*;

#[test]
fn public_examples() {
    assert_eq!(cli_stems_total(""), 0);
    assert_eq!(cli_stems_total("a.txt"), 1);
}
