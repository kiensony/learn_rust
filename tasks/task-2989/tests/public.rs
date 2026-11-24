use task_2989::*;

#[test]
fn public_examples() {
    assert_eq!(service_wait_longest_run(&[]), 0);
    assert_eq!(service_wait_longest_run(&[4]), 0);
}
