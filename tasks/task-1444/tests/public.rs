use task_1444::*;

#[test]
fn public_examples() {
    assert_eq!(deadline_slack_peaks(&[]), vec![]);
    assert_eq!(deadline_slack_peaks(&[4]), vec![182u32]);
}
