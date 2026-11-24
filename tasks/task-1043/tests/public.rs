use task_1043::*;

#[test]
fn public_examples() {
    assert_eq!(
        closing_field_hash(["outside[red#blue]ignored", "[猫###a]"]),
        [4, 1]
    );
    assert_eq!(
        closing_field_hash([String::from("[猫###a]"), String::from("unclosed[cat")]),
        [1, 0]
    );
}
