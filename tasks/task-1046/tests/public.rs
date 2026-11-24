use task_1046::*;

#[test]
fn public_examples() {
    assert_eq!(
        marks_removed_at_both_ends_hash(["outside[red#blue]ignored", "[猫###a]"]),
        [8, 7]
    );
    assert_eq!(
        marks_removed_at_both_ends_hash([String::from("[猫###a]"), String::from("unclosed[cat")]),
        [7, 0]
    );
}
