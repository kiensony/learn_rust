use task_389::*;

#[test]
fn public_examples() {
    assert_eq!(closing_field_equals(&String::from("72;12")), 2);
    assert_eq!(closing_field_equals(&String::from("hi=kit")), 3);
}
