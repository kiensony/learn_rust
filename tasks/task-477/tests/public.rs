use task_477::*;

#[test]
fn public_examples() {
    assert_eq!(shortest_field_with_first_tie_slash(("path/1/12", "x")), 1);
    assert_eq!(shortest_field_with_first_tie_slash(("path/123", "z/w")), 3);
}
