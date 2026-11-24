use task_1302::*;

#[test]
fn public_examples() {
    assert_eq!(decimal_fields_total(""), 0);
    assert_eq!(decimal_fields_total("1"), 1);
}
