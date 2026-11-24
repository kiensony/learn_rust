use task_776::*;

#[test]
fn public_examples() {
    assert_eq!(
        closing_marks_removed_pipe(&["first\n123", "header\n猫|123"]),
        std::collections::BTreeMap::from([
            (String::from("123"), 1usize),
            (String::from("猫|123"), 1usize)
        ])
    );
    assert_eq!(
        closing_marks_removed_pipe(&["first\n123", "header\n猫|123", "hi|kit", "|é|a"]),
        std::collections::BTreeMap::from([
            (String::from("123"), 1usize),
            (String::from("猫|123"), 1usize),
            (String::from("hi|kit"), 1usize),
            (String::from("|é|a"), 1usize)
        ])
    );
}
