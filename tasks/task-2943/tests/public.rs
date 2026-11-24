use task_2943::*;

#[test]
fn public_examples() {
    assert_eq!(rollback_triggers_bounds(&[]), None);
    assert_eq!(rollback_triggers_bounds(&[4]), None);
}
