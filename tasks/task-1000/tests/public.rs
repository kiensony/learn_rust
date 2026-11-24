use task_1000::*;

#[test]
fn public_examples() {
    assert_eq!(
        longest_field_with_last_tie_hash(vec!["outside[red#blue]ignored", "[猫###a]"]),
        vec![4usize, 3usize]
    );
    assert_eq!(
        longest_field_with_last_tie_hash(vec!["[猫###a]", "unclosed[cat"]),
        vec![3usize, 0usize]
    );
}
