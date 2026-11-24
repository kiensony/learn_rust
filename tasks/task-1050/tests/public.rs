use task_1050::*;

#[test]
fn public_examples() {
    assert_eq!(
        longest_field_with_last_tie_hash(["outside[red#blue]ignored", "[猫###a]"]),
        [4, 3]
    );
    assert_eq!(
        longest_field_with_last_tie_hash([String::from("[猫###a]"), String::from("unclosed[cat")]),
        [3, 0]
    );
}
