use task_405::*;

#[test]
fn public_examples() {
    assert_eq!(
        first_nonempty_field_hash(&String::from("outside[red#blue]ignored")),
        3
    );
    assert_eq!(first_nonempty_field_hash(&String::from("unclosed[cat")), 0);
}
