use task_1989::*;

#[test]
fn public_examples() {
    assert_eq!(atomic_record_high_longest_run(&[]), 0);
    assert_eq!(atomic_record_high_longest_run(&[4]), 1);
}
