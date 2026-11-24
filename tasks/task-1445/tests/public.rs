use task_1445::*;

#[test]
fn public_examples() {
    assert_eq!(deadline_slack_changes(&[]), vec![]);
    assert_eq!(deadline_slack_changes(&[4]), vec![182u32]);
}
