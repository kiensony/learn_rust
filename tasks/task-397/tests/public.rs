use task_397::*;

#[test]
fn public_examples() {
    assert_eq!(
        shortest_field_with_first_tie_equals(&String::from("72;12")),
        2
    );
    assert_eq!(
        shortest_field_with_first_tie_equals(&String::from("=é=a")),
        0
    );
}
