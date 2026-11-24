use task_769::*;

#[test]
fn public_examples() {
    assert_eq!(
        prefix_before_final_mark_slash(&["path/1/12", "path/123"]),
        std::collections::BTreeMap::from([(String::from("1"), 1usize), (String::from(""), 1usize)])
    );
    assert_eq!(
        prefix_before_final_mark_slash(&["path/1/12", "path/123", "hi/kit", "/é/a"]),
        std::collections::BTreeMap::from([
            (String::from("1"), 1usize),
            (String::from(""), 1usize),
            (String::from("hi"), 1usize),
            (String::from("/é"), 1usize)
        ])
    );
}
