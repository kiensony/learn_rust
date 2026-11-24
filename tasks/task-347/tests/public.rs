use task_347::*;

#[test]
fn public_examples() {
    assert_eq!(
        shortest_field_with_first_tie_equals(String::from("72;12")),
        "72"
    );
    assert_eq!(
        shortest_field_with_first_tie_equals(String::from("猫=12;note")),
        "12"
    );
}
