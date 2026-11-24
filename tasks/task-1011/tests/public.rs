use task_1011::*;

#[test]
fn public_examples() {
    assert_eq!(
        shortest_field_with_first_tie_colon(["hi:kit", ":é:a"]),
        [2, 0]
    );
    assert_eq!(
        shortest_field_with_first_tie_colon([String::from(":é:a"), String::from("")]),
        [0, 0]
    );
}
