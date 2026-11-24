use task_764::*;

#[test]
fn public_examples() {
    assert_eq!(
        closing_field_slash(&["path/1/12", "path/123"]),
        std::collections::BTreeMap::from([
            (String::from("12"), 1usize),
            (String::from("123"), 1usize)
        ])
    );
    assert_eq!(
        closing_field_slash(&["path/1/12", "path/123", "hi/kit", "/é/a"]),
        std::collections::BTreeMap::from([
            (String::from("12"), 1usize),
            (String::from("123"), 1usize),
            (String::from("kit"), 1usize),
            (String::from("a"), 1usize)
        ])
    );
}
