use task_767::*;

#[test]
fn public_examples() {
    assert_eq!(
        marks_removed_at_both_ends_slash(&["path/1/12", "path/123"]),
        std::collections::BTreeMap::from([
            (String::from("1/12"), 1usize),
            (String::from("123"), 1usize)
        ])
    );
    assert_eq!(
        marks_removed_at_both_ends_slash(&["path/1/12", "path/123", "hi/kit", "/é/a"]),
        std::collections::BTreeMap::from([
            (String::from("1/12"), 1usize),
            (String::from("123"), 1usize),
            (String::from("hi/kit"), 1usize),
            (String::from("é/a"), 1usize)
        ])
    );
}
