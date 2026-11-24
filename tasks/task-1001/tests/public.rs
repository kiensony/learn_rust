use task_1001::*;

#[test]
fn public_examples() {
    assert_eq!(
        shortest_field_with_first_tie_hash(vec!["outside[red#blue]ignored", "[猫###a]"]),
        vec![3usize, 0usize]
    );
    assert_eq!(
        shortest_field_with_first_tie_hash(vec!["[猫###a]", "unclosed[cat"]),
        vec![0usize, 0usize]
    );
}
