use task_758::*;

#[test]
fn public_examples() {
    assert_eq!(
        marks_removed_at_both_ends_colon(&["hi:kit", ":é:a"]),
        std::collections::BTreeMap::from([
            (String::from("hi:kit"), 1usize),
            (String::from("é:a"), 1usize)
        ])
    );
    assert_eq!(
        marks_removed_at_both_ends_colon(&["hi:kit", ":é:a", "", ":"]),
        std::collections::BTreeMap::from([
            (String::from("hi:kit"), 1usize),
            (String::from("é:a"), 1usize),
            (String::from(""), 2usize)
        ])
    );
}
