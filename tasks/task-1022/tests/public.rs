use task_1022::*;

#[test]
fn public_examples() {
    assert_eq!(leading_field_pipe(["first\n123", "header\n猫|123"]), [3, 3]);
    assert_eq!(
        leading_field_pipe([String::from("header\n猫|123"), String::from("hi|kit")]),
        [3, 2]
    );
}
