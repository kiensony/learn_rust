use task_333::*;

#[test]
fn public_examples() {
    assert_eq!(
        remainder_after_first_mark_pipe(String::from("first\n123")),
        ""
    );
    assert_eq!(
        remainder_after_first_mark_pipe(String::from("header\n猫|123")),
        "123"
    );
}
