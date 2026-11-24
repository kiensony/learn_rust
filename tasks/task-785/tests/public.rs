use task_785::*;

#[test]
fn public_examples() {
    assert_eq!(
        leading_marks_removed_equals(&["72;12", "猫=12;note"]),
        std::collections::BTreeMap::from([
            (String::from("72"), 1usize),
            (String::from("猫=12"), 1usize)
        ])
    );
    assert_eq!(
        leading_marks_removed_equals(&["72;12", "猫=12;note", "hi=kit", "=é=a"]),
        std::collections::BTreeMap::from([
            (String::from("72"), 1usize),
            (String::from("猫=12"), 1usize),
            (String::from("hi=kit"), 1usize),
            (String::from("é=a"), 1usize)
        ])
    );
}
