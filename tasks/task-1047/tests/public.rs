use task_1047::*;

#[test]
fn public_examples() {
    assert_eq!(
        remainder_after_first_mark_hash(["outside[red#blue]ignored", "[猫###a]"]),
        [4, 3]
    );
    assert_eq!(
        remainder_after_first_mark_hash([String::from("[猫###a]"), String::from("unclosed[cat")]),
        [3, 0]
    );
}
