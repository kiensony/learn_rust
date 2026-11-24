use task_997::*;

#[test]
fn public_examples() {
    assert_eq!(
        remainder_after_first_mark_hash(vec!["outside[red#blue]ignored", "[猫###a]"]),
        vec![4usize, 3usize]
    );
    assert_eq!(
        remainder_after_first_mark_hash(vec!["[猫###a]", "unclosed[cat"]),
        vec![3usize, 0usize]
    );
}
