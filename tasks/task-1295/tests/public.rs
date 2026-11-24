use task_1295::*;

#[test]
fn public_examples() {
    assert_eq!(quiet_option_positions_changes(""), vec![]);
    assert_eq!(quiet_option_positions_changes("only"), vec![]);
}
