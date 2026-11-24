use task_2809::*;

#[test]
fn public_examples() {
    assert_eq!(service_acknowledgments_longest_run(&[]), 0);
    assert_eq!(service_acknowledgments_longest_run(&[4]), 1);
}
