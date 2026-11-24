use task_2112::*;

#[test]
fn public_examples() {
    assert_eq!(priority_removal_total(&[]), 0);
    assert_eq!(priority_removal_total(&[4]), 4);
}
