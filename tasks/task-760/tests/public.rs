use task_760::*;

#[test]
fn public_examples() {
    assert_eq!(
        first_nonempty_field_colon(&["hi:kit", ":é:a"]),
        std::collections::BTreeMap::from([
            (String::from("hi"), 1usize),
            (String::from("é"), 1usize)
        ])
    );
    assert_eq!(
        first_nonempty_field_colon(&["hi:kit", ":é:a", "", ":"]),
        std::collections::BTreeMap::from([
            (String::from("hi"), 1usize),
            (String::from("é"), 1usize),
            (String::from(""), 2usize)
        ])
    );
}
