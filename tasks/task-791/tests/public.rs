use task_791::*;

#[test]
fn public_examples() {
    assert_eq!(
        longest_field_with_last_tie_equals(&["72;12", "猫=12;note"]),
        std::collections::BTreeMap::from([
            (String::from("72"), 1usize),
            (String::from("猫"), 1usize)
        ])
    );
    assert_eq!(
        longest_field_with_last_tie_equals(&["72;12", "猫=12;note", "hi=kit", "=é=a"]),
        std::collections::BTreeMap::from([
            (String::from("72"), 1usize),
            (String::from("猫"), 1usize),
            (String::from("kit"), 1usize),
            (String::from("é"), 1usize)
        ])
    );
}
