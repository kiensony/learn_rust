use task_759::*;

#[test]
fn public_examples() {
    assert_eq!(
        prefix_before_final_mark_colon(&["hi:kit", ":é:a"]),
        std::collections::BTreeMap::from([
            (String::from("hi"), 1usize),
            (String::from(":é"), 1usize)
        ])
    );
    assert_eq!(
        prefix_before_final_mark_colon(&["hi:kit", ":é:a", "", ":"]),
        std::collections::BTreeMap::from([
            (String::from("hi"), 1usize),
            (String::from(":é"), 1usize),
            (String::from(""), 2usize)
        ])
    );
}
