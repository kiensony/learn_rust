use task_992::*;

#[test]
fn public_examples() {
    assert_eq!(
        leading_field_hash(vec!["outside[red#blue]ignored", "[猫###a]"]),
        vec![3usize, 3usize]
    );
    assert_eq!(
        leading_field_hash(vec!["[猫###a]", "unclosed[cat"]),
        vec![3usize, 0usize]
    );
}
