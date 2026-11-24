use task_762::*;

#[test]
fn public_examples() {
    assert_eq!(
        shortest_field_with_first_tie_colon(&["hi:kit", ":é:a"]),
        std::collections::BTreeMap::from([
            (String::from("hi"), 1usize),
            (String::from(""), 1usize)
        ])
    );
    assert_eq!(
        shortest_field_with_first_tie_colon(&["hi:kit", ":é:a", "", ":"]),
        std::collections::BTreeMap::from([
            (String::from("hi"), 1usize),
            (String::from(""), 3usize)
        ])
    );
}
