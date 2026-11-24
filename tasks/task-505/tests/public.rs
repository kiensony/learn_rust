use task_505::*;

#[test]
fn public_examples() {
    assert_eq!(
        first_nonempty_field_hash(("outside[red#blue]ignored", "x")),
        3
    );
    assert_eq!(first_nonempty_field_hash(("unclosed[cat", "x")), 0);
}
