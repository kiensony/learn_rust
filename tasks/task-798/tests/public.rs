use task_798::*;

#[test]
fn public_examples() {
    assert_eq!(
        remainder_after_first_mark_hash(&["outside[red#blue]ignored", "[猫###a]"]),
        std::collections::BTreeMap::from([
            (String::from("blue"), 1usize),
            (String::from("##a"), 1usize)
        ])
    );
    assert_eq!(
        remainder_after_first_mark_hash(&[
            "outside[red#blue]ignored",
            "[猫###a]",
            "unclosed[cat",
            "[]"
        ]),
        std::collections::BTreeMap::from([
            (String::from("blue"), 1usize),
            (String::from("##a"), 1usize),
            (String::from(""), 2usize)
        ])
    );
}
