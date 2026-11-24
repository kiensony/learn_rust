use task_1442::*;

#[test]
fn public_examples() {
    assert_eq!(deadline_slack_total(&[]), 0);
    assert_eq!(deadline_slack_total(&[4]), 182);
}
