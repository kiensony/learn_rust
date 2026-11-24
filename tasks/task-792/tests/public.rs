use task_792::*;

#[test]
fn public_examples() {
    assert_eq!(
        shortest_field_with_first_tie_equals(&["72;12", "猫=12;note"]),
        std::collections::BTreeMap::from([
            (String::from("72"), 1usize),
            (String::from("12"), 1usize)
        ])
    );
    assert_eq!(
        shortest_field_with_first_tie_equals(&["72;12", "猫=12;note", "hi=kit", "=é=a"]),
        std::collections::BTreeMap::from([
            (String::from("72"), 1usize),
            (String::from("12"), 1usize),
            (String::from("hi"), 1usize),
            (String::from(""), 1usize)
        ])
    );
}
