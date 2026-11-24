use task_2945::*;

#[test]
fn public_examples() {
    assert_eq!(rollback_triggers_changes(&[]), vec![]);
    assert_eq!(rollback_triggers_changes(&[4]), vec![]);
}
