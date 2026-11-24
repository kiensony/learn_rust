use task_398::*;

#[test]
fn public_examples() {
    assert_eq!(
        leading_field_hash(&String::from("outside[red#blue]ignored")),
        3
    );
    assert_eq!(leading_field_hash(&String::from("unclosed[cat")), 0);
}
