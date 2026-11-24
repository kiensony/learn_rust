use task_1160::*;

#[test]
fn public_examples() {
    assert_eq!(
        shortest_field_with_first_tie_colon("hi:kit", "backup"),
        "hi"
    );
    assert_eq!(
        shortest_field_with_first_tie_colon(":é:a", "backup"),
        "backup"
    );
}
