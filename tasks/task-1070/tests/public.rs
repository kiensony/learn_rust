use task_1070::*;

#[test]
fn public_examples() {
    assert_eq!(shortest_field_with_first_tie_slash(&"path/1/12"), 1);
    assert_eq!(
        shortest_field_with_first_tie_slash(&String::from("path/123")),
        3
    );
}
