use task_891::*;

#[test]
fn public_examples() {
    assert_eq!(longest_field_with_last_tie_equals("72;12"), 4);
    assert_eq!(longest_field_with_last_tie_equals("猫=12;note"), 5);
}
