use task_1021::*;

#[test]
fn public_examples() {
    assert_eq!(
        shortest_field_with_first_tie_slash(["path/1/12", "path/123"]),
        [1, 3]
    );
    assert_eq!(
        shortest_field_with_first_tie_slash([String::from("path/123"), String::from("hi/kit")]),
        [3, 2]
    );
}
