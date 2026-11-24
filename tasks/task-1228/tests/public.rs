use task_1228::*;

#[test]
fn public_examples() {
    assert_eq!(delimited_fields_steps(&[]), vec![]);
    assert_eq!(delimited_fields_steps(&[4]), vec![]);
}
