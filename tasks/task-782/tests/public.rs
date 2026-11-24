use task_782::*;

#[test]
fn public_examples() {
    assert_eq!(
        shortest_field_with_first_tie_pipe(&["first\n123", "header\n猫|123"]),
        std::collections::BTreeMap::from([
            (String::from("123"), 1usize),
            (String::from("猫"), 1usize)
        ])
    );
    assert_eq!(
        shortest_field_with_first_tie_pipe(&["first\n123", "header\n猫|123", "hi|kit", "|é|a"]),
        std::collections::BTreeMap::from([
            (String::from("123"), 1usize),
            (String::from("猫"), 1usize),
            (String::from("hi"), 1usize),
            (String::from(""), 1usize)
        ])
    );
}
