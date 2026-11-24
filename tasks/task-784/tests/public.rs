use task_784::*;

#[test]
fn public_examples() {
    assert_eq!(
        closing_field_equals(&["72;12", "猫=12;note"]),
        std::collections::BTreeMap::from([
            (String::from("72"), 1usize),
            (String::from("12"), 1usize)
        ])
    );
    assert_eq!(
        closing_field_equals(&["72;12", "猫=12;note", "hi=kit", "=é=a"]),
        std::collections::BTreeMap::from([
            (String::from("72"), 1usize),
            (String::from("12"), 1usize),
            (String::from("kit"), 1usize),
            (String::from("a"), 1usize)
        ])
    );
}
