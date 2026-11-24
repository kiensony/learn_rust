use task_1229::*;

#[test]
fn public_examples() {
    assert_eq!(delimited_fields_longest_run(&[]), 0);
    assert_eq!(delimited_fields_longest_run(&[4]), 1);
}
