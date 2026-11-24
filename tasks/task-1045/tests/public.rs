use task_1045::*;

#[test]
fn public_examples() {
    assert_eq!(
        closing_marks_removed_hash(["outside[red#blue]ignored", "[猫###a]"]),
        [8, 7]
    );
    assert_eq!(
        closing_marks_removed_hash([String::from("[猫###a]"), String::from("unclosed[cat")]),
        [7, 0]
    );
}
