use task_848::*;

#[test]
fn public_examples() {
    assert_eq!(
        remainder_after_first_mark_hash("outside[red#blue]ignored"),
        Ok(4)
    );
    assert_eq!(remainder_after_first_mark_hash("[猫###a]"), Ok(3));
}
