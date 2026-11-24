use task_755::*;

#[test]
fn public_examples() {
    assert_eq!(
        closing_field_colon(&["hi:kit", ":é:a"]),
        std::collections::BTreeMap::from([
            (String::from("kit"), 1usize),
            (String::from("a"), 1usize)
        ])
    );
    assert_eq!(
        closing_field_colon(&["hi:kit", ":é:a", "", ":"]),
        std::collections::BTreeMap::from([
            (String::from("kit"), 1usize),
            (String::from("a"), 1usize),
            (String::from(""), 2usize)
        ])
    );
}
