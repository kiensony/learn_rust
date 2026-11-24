use task_768::*;

#[test]
fn public_examples() {
    assert_eq!(
        remainder_after_first_mark_slash(&["path/1/12", "path/123"]),
        std::collections::BTreeMap::from([
            (String::from("12"), 1usize),
            (String::from(""), 1usize)
        ])
    );
    assert_eq!(
        remainder_after_first_mark_slash(&["path/1/12", "path/123", "hi/kit", "/é/a"]),
        std::collections::BTreeMap::from([
            (String::from("12"), 1usize),
            (String::from(""), 1usize),
            (String::from("kit"), 1usize),
            (String::from("é/a"), 1usize)
        ])
    );
}
