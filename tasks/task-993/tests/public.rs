use task_993::*;

#[test]
fn public_examples() {
    assert_eq!(
        closing_field_hash(vec!["outside[red#blue]ignored", "[猫###a]"]),
        vec![4usize, 1usize]
    );
    assert_eq!(
        closing_field_hash(vec!["[猫###a]", "unclosed[cat"]),
        vec![1usize, 0usize]
    );
}
