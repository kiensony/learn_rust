use task_2708::*;

#[test]
fn public_examples() {
    assert_eq!(cli_basenames_steps(""), vec![]);
    assert_eq!(cli_basenames_steps("a.txt"), vec![]);
}
