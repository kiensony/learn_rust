use task_1291::*;

#[test]
fn public_examples() {
    assert_eq!(quiet_option_positions_trace(""), vec![]);
    assert_eq!(quiet_option_positions_trace("only"), vec![]);
}
