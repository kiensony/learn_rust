use task_772::*;

#[test]
fn public_examples() {
    assert_eq!(
        shortest_field_with_first_tie_slash(&["path/1/12", "path/123"]),
        std::collections::BTreeMap::from([
            (String::from("1"), 1usize),
            (String::from("123"), 1usize)
        ])
    );
    assert_eq!(
        shortest_field_with_first_tie_slash(&["path/1/12", "path/123", "hi/kit", "/é/a"]),
        std::collections::BTreeMap::from([
            (String::from("1"), 1usize),
            (String::from("123"), 1usize),
            (String::from("hi"), 1usize),
            (String::from(""), 1usize)
        ])
    );
}
