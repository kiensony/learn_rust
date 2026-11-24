use task_1222::*;

#[test]
fn public_examples() {
    assert_eq!(delimited_fields_total(&[]), 0);
    assert_eq!(delimited_fields_total(&[4]), 1);
}
