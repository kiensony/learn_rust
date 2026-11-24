use task_771::*;

#[test]
fn public_examples() {
    assert_eq!(
        longest_field_with_last_tie_slash(&["path/1/12", "path/123"]),
        std::collections::BTreeMap::from([
            (String::from("12"), 1usize),
            (String::from("123"), 1usize)
        ])
    );
    assert_eq!(
        longest_field_with_last_tie_slash(&["path/1/12", "path/123", "hi/kit", "/é/a"]),
        std::collections::BTreeMap::from([
            (String::from("12"), 1usize),
            (String::from("123"), 1usize),
            (String::from("kit"), 1usize),
            (String::from("é"), 1usize)
        ])
    );
}
