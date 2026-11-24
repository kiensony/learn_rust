use task_2819::*;

#[test]
fn public_examples() {
    assert_eq!(service_validation_longest_run(&[]), 0);
    assert_eq!(service_validation_longest_run(&[4]), 1);
}
