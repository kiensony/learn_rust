use task_1032::*;

#[test]
fn public_examples() {
    assert_eq!(leading_field_equals(["72;12", "猫=12;note"]), [2, 3]);
    assert_eq!(
        leading_field_equals([String::from("猫=12;note"), String::from("hi=kit")]),
        [3, 2]
    );
}
