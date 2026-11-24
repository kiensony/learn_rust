use task_996::*;

#[test]
fn public_examples() {
    assert_eq!(
        marks_removed_at_both_ends_hash(vec!["outside[red#blue]ignored", "[猫###a]"]),
        vec![8usize, 7usize]
    );
    assert_eq!(
        marks_removed_at_both_ends_hash(vec!["[猫###a]", "unclosed[cat"]),
        vec![7usize, 0usize]
    );
}
