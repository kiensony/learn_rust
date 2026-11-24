use task_780::*;

#[test]
fn public_examples() {
    assert_eq!(
        first_nonempty_field_pipe(&["first\n123", "header\n猫|123"]),
        std::collections::BTreeMap::from([
            (String::from("123"), 1usize),
            (String::from("猫"), 1usize)
        ])
    );
    assert_eq!(
        first_nonempty_field_pipe(&["first\n123", "header\n猫|123", "hi|kit", "|é|a"]),
        std::collections::BTreeMap::from([
            (String::from("123"), 1usize),
            (String::from("猫"), 1usize),
            (String::from("hi"), 1usize),
            (String::from("é"), 1usize)
        ])
    );
}
