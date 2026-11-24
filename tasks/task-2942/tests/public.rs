use task_2942::*;

#[test]
fn public_examples() {
    assert_eq!(rollback_triggers_total(&[]), 0);
    assert_eq!(rollback_triggers_total(&[4]), 0);
}
