use task_2702::*;

#[test]
fn public_examples() {
    assert_eq!(cli_basenames_total(""), 0);
    assert_eq!(cli_basenames_total("a.txt"), 5);
}
