use task_1082::*;

#[test]
fn public_examples() {
    assert_eq!(closing_field_equals(&"72;12"), 2);
    assert_eq!(closing_field_equals(&"hi=kit"), 3);
}
