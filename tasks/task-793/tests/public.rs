use task_793::*;

#[test]
fn public_examples() {
    assert_eq!(
        leading_field_hash(&["outside[red#blue]ignored", "[猫###a]"]),
        std::collections::BTreeMap::from([
            (String::from("red"), 1usize),
            (String::from("猫"), 1usize)
        ])
    );
    assert_eq!(
        leading_field_hash(&["outside[red#blue]ignored", "[猫###a]", "unclosed[cat", "[]"]),
        std::collections::BTreeMap::from([
            (String::from("red"), 1usize),
            (String::from("猫"), 1usize),
            (String::from(""), 2usize)
        ])
    );
}
