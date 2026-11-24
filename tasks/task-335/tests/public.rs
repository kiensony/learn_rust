use task_335::*;

#[test]
fn public_examples() {
    assert_eq!(first_nonempty_field_pipe(String::from("first\n123")), "123");
    assert_eq!(
        first_nonempty_field_pipe(String::from("header\n猫|123")),
        "猫"
    );
}
