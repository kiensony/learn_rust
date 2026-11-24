use task_596::*;

#[test]
fn public_examples() {
    assert_eq!(shortest_field_with_first_tie_equals("72;12"), "72");
    assert_eq!(shortest_field_with_first_tie_equals("猫=12;note"), "12");
}
