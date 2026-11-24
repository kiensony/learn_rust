use task_761::*;

#[test]
fn public_examples() {
    assert_eq!(
        longest_field_with_last_tie_colon(&["hi:kit", ":é:a"]),
        std::collections::BTreeMap::from([
            (String::from("kit"), 1usize),
            (String::from("é"), 1usize)
        ])
    );
    assert_eq!(
        longest_field_with_last_tie_colon(&["hi:kit", ":é:a", "", ":"]),
        std::collections::BTreeMap::from([
            (String::from("kit"), 1usize),
            (String::from("é"), 1usize),
            (String::from(""), 2usize)
        ])
    );
}
