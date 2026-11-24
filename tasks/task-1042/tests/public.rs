use task_1042::*;

#[test]
fn public_examples() {
    assert_eq!(
        leading_field_hash(["outside[red#blue]ignored", "[猫###a]"]),
        [3, 3]
    );
    assert_eq!(
        leading_field_hash([String::from("[猫###a]"), String::from("unclosed[cat")]),
        [3, 0]
    );
}
