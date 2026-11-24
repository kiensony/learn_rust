use task_2706::*;

#[test]
fn public_examples() {
    assert_eq!(cli_basenames_descending_steps(""), 0);
    assert_eq!(cli_basenames_descending_steps("a.txt"), 0);
}
