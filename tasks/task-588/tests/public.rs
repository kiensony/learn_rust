use task_588::*;

#[test]
fn public_examples() {
    assert_eq!(closing_field_equals("72;12"), "72");
    assert_eq!(closing_field_equals("猫=12;note"), "12");
}
