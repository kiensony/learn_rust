use task_781::*;

#[test]
fn public_examples() {
    assert_eq!(
        longest_field_with_last_tie_pipe(&["first\n123", "header\n猫|123"]),
        std::collections::BTreeMap::from([(String::from("123"), 2usize)])
    );
    assert_eq!(
        longest_field_with_last_tie_pipe(&["first\n123", "header\n猫|123", "hi|kit", "|é|a"]),
        std::collections::BTreeMap::from([
            (String::from("123"), 2usize),
            (String::from("kit"), 1usize),
            (String::from("é"), 1usize)
        ])
    );
}
