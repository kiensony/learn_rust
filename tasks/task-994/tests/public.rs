use task_994::*;

#[test]
fn public_examples() {
    assert_eq!(
        leading_marks_removed_hash(vec!["outside[red#blue]ignored", "[猫###a]"]),
        vec![8usize, 7usize]
    );
    assert_eq!(
        leading_marks_removed_hash(vec!["[猫###a]", "unclosed[cat"]),
        vec![7usize, 0usize]
    );
}
