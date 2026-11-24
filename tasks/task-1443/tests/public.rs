use task_1443::*;

#[test]
fn public_examples() {
    assert_eq!(deadline_slack_bounds(&[]), None);
    assert_eq!(deadline_slack_bounds(&[4]), Some((182u32, 182u32)));
}
