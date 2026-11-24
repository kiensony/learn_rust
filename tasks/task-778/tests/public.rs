use task_778::*;

#[test]
fn public_examples() {
    assert_eq!(
        remainder_after_first_mark_pipe(&["first\n123", "header\n猫|123"]),
        std::collections::BTreeMap::from([
            (String::from(""), 1usize),
            (String::from("123"), 1usize)
        ])
    );
    assert_eq!(
        remainder_after_first_mark_pipe(&["first\n123", "header\n猫|123", "hi|kit", "|é|a"]),
        std::collections::BTreeMap::from([
            (String::from(""), 1usize),
            (String::from("123"), 1usize),
            (String::from("kit"), 1usize),
            (String::from("é|a"), 1usize)
        ])
    );
}
