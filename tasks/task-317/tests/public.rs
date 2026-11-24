use task_317::*;

#[test]
fn public_examples() {
    assert_eq!(
        shortest_field_with_first_tie_colon(String::from("hi:kit")),
        "hi"
    );
    assert_eq!(
        shortest_field_with_first_tie_colon(String::from(":é:a")),
        ""
    );
}
