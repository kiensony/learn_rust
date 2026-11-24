use task_367::*;

#[test]
fn public_examples() {
    assert_eq!(
        shortest_field_with_first_tie_colon(&String::from("hi:kit")),
        2
    );
    assert_eq!(
        shortest_field_with_first_tie_colon(&String::from(":é:a")),
        0
    );
}
