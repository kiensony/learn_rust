use task_337::*;

#[test]
fn public_examples() {
    assert_eq!(
        shortest_field_with_first_tie_pipe(String::from("first\n123")),
        "123"
    );
    assert_eq!(
        shortest_field_with_first_tie_pipe(String::from("header\n猫|123")),
        "猫"
    );
}
