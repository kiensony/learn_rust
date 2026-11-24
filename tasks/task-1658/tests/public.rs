use task_1658::*;

#[test]
fn public_examples() {
    assert_eq!(major_version_steps(""), vec![]);
    assert_eq!(major_version_steps("0.0.1"), vec![]);
}
