use task_754::*;

#[test]
fn public_examples() {
    assert_eq!(
        leading_field_colon(&["hi:kit", ":é:a"]),
        std::collections::BTreeMap::from([
            (String::from("hi"), 1usize),
            (String::from(""), 1usize)
        ])
    );
    assert_eq!(
        leading_field_colon(&["hi:kit", ":é:a", "", ":"]),
        std::collections::BTreeMap::from([
            (String::from("hi"), 1usize),
            (String::from(""), 3usize)
        ])
    );
}
