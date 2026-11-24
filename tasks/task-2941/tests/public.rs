use task_2941::*;

#[test]
fn public_examples() {
    assert_eq!(rollback_triggers_trace(&[]), vec![]);
    assert_eq!(rollback_triggers_trace(&[4]), vec![]);
}
