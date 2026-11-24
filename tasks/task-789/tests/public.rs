use task_789::*;

#[test]
fn public_examples() {
    assert_eq!(
        prefix_before_final_mark_equals(&["72;12", "猫=12;note"]),
        std::collections::BTreeMap::from([
            (String::from(""), 1usize),
            (String::from("猫"), 1usize)
        ])
    );
    assert_eq!(
        prefix_before_final_mark_equals(&["72;12", "猫=12;note", "hi=kit", "=é=a"]),
        std::collections::BTreeMap::from([
            (String::from(""), 1usize),
            (String::from("猫"), 1usize),
            (String::from("hi"), 1usize),
            (String::from("=é"), 1usize)
        ])
    );
}
