use task_1441::*;

#[test]
fn public_examples() {
    assert_eq!(deadline_slack_trace(&[]), vec![]);
    assert_eq!(deadline_slack_trace(&[4]), vec![182u32]);
}
