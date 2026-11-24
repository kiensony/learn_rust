use task_788::*;

#[test]
fn public_examples() {
    assert_eq!(
        remainder_after_first_mark_equals(&["72;12", "猫=12;note"]),
        std::collections::BTreeMap::from([
            (String::from(""), 1usize),
            (String::from("12"), 1usize)
        ])
    );
    assert_eq!(
        remainder_after_first_mark_equals(&["72;12", "猫=12;note", "hi=kit", "=é=a"]),
        std::collections::BTreeMap::from([
            (String::from(""), 1usize),
            (String::from("12"), 1usize),
            (String::from("kit"), 1usize),
            (String::from("é=a"), 1usize)
        ])
    );
}
