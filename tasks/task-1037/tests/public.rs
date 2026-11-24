use task_1037::*;

#[test]
fn public_examples() {
    assert_eq!(
        remainder_after_first_mark_equals(["72;12", "猫=12;note"]),
        [0, 2]
    );
    assert_eq!(
        remainder_after_first_mark_equals([String::from("猫=12;note"), String::from("hi=kit")]),
        [2, 3]
    );
}
