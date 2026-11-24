use task_757::*;

#[test]
fn public_examples() {
    assert_eq!(
        closing_marks_removed_colon(&["hi:kit", ":é:a"]),
        std::collections::BTreeMap::from([
            (String::from("hi:kit"), 1usize),
            (String::from(":é:a"), 1usize)
        ])
    );
    assert_eq!(
        closing_marks_removed_colon(&["hi:kit", ":é:a", "", ":"]),
        std::collections::BTreeMap::from([
            (String::from("hi:kit"), 1usize),
            (String::from(":é:a"), 1usize),
            (String::from(""), 2usize)
        ])
    );
}
