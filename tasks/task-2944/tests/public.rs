use task_2944::*;

#[test]
fn public_examples() {
    assert_eq!(rollback_triggers_peaks(&[]), vec![]);
    assert_eq!(rollback_triggers_peaks(&[4]), vec![]);
}
