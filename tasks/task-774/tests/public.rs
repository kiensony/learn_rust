use task_774::*;

#[test]
fn public_examples() {
    assert_eq!(
        closing_field_pipe(&["first\n123", "header\n猫|123"]),
        std::collections::BTreeMap::from([(String::from("123"), 2usize)])
    );
    assert_eq!(
        closing_field_pipe(&["first\n123", "header\n猫|123", "hi|kit", "|é|a"]),
        std::collections::BTreeMap::from([
            (String::from("123"), 2usize),
            (String::from("kit"), 1usize),
            (String::from("a"), 1usize)
        ])
    );
}
